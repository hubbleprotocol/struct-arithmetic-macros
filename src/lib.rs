use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput,
    Field, Fields, Ident, Path, Type, TypePath,
};

#[proc_macro_derive(StructArithmetic, attributes(helper))]
pub fn struct_arithmetic(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let fields: Punctuated<Field, Comma> = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with named fields can be annotated with ToUrl"),
    };

    let field_type = match &fields.first().unwrap().ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            if let Some(path_seg) = segments.first() {
                let ident: &proc_macro2::Ident = &path_seg.ident;
                Some(ident.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
    .unwrap();

    let factor = Ident::new("factor", Span::call_site());
    let numerator = Ident::new("numerator", Span::call_site());
    let denominator = Ident::new("denominator", Span::call_site());
    let fields_type = Ident::new(&field_type, Span::call_site());

    let (addition, addition_array) = generate_add(&fields);
    let addition_assign = generate_add_assign(&fields);
    let (subtraction, subtraction_array) = generate_sub(&fields);
    let subtraction_assign = generate_sub_assign(&fields);
    let (multiplication, multiplication_array) = generate_mul(&fields);
    let (division, division_array) = generate_div(&fields);
    let (division_scalar, division_scalar_array) = generate_div_scalar(&fields, factor.clone());
    let (multiplication_scalar, multiplication_scalar_array) =
        generate_mul_scalar(&fields, factor.clone());
    let (multiplication_fraction, multiplication_fraction_array) =
        generate_mul_fraction(&fields, numerator, denominator, fields_type.clone());

    let (new_constructor_args, new_constructor_struct) = generate_new(&fields);
    let is_zero = generate_is_zero(&fields);
    // let token_amount = generate_token_amount(&fields);

    let modified = quote! {
        impl #name {
            pub fn new(#(#new_constructor_args)*) -> #name {
                #name {
                #(#new_constructor_struct)*
                }
            }

            pub fn is_zero(&self) -> bool {
                #(#is_zero)*
                return true;
            }

            pub fn add(&self, other: &#name) -> Option<#name> {
                #(#addition_array)*
                Some(#name::new(
                    #(#addition)*
                ))
            }

            pub fn add_assign(&mut self, other: &#name) -> Option<()> {
                #(#addition_assign)*

                Some(())
            }

            pub fn sub(&self, other: &#name) -> Option<#name> {
                #(#subtraction_array)*
                Some(#name::new(
                    #(#subtraction)*
                ))
            }

            pub fn sub_assign(&mut self, other: &#name) -> Option<()> {
                #(#subtraction_assign)*

                Some(())
            }

            pub fn div(&self, other: &#name) -> Option<#name> {
                #(#division_array)*
                Some(#name::new(
                    #(#division)*
                ))
            }

            pub fn div_scalar(&self, factor: #fields_type) -> Option<#name> {
                #(#division_scalar_array)*
                Some(#name::new(
                    #(#division_scalar)*
                ))
            }

            pub fn mul(&self, other: &#name) -> Option<#name> {
                #(#multiplication_array)*
                Some(#name::new(
                    #(#multiplication)*
                ))
            }

            pub fn mul_scalar(&self, factor: #fields_type) -> Option<#name> {
                #(#multiplication_scalar_array)*
                Some(#name::new(
                    #(#multiplication_scalar)*
                ))
            }

            pub fn mul_fraction(&self, numerator: #fields_type, denominator: #fields_type) -> Option<#name> {
                #(#multiplication_fraction_array)*
                Some(#name::new(
                    #(#multiplication_fraction)*
                ))
            }

            pub fn mul_bps(&self, factor: u16) -> Option<#name> {
                self.mul_fraction(factor as #fields_type, 10_000)
            }


            pub fn mul_percent(&self, factor: u16) -> Option<#name> {
                self.mul_fraction(factor as #fields_type, 100)
            }

        }
    };
    TokenStream::from(modified)
}

//     args_code
// }

fn generate_is_zero<'a>(
    fields: &'a Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    let args_code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            quote! {
                for i in 0..self.#field_ident.len() {
                    if self.#field_ident[i] != 0 {
                        return false;
                    }
                }
            }
        });
    let args_code = fields
        .into_iter()
        .filter(|field| match &field.ty {
            Type::Array(_arr) => false,
            _ => true,
        })
        .map(move |field| {
            let field_ident = field.ident.as_ref().unwrap();
            quote! {
                if self.#field_ident != 0 {
                    return false;
                }
            }
        });

    args_code.chain(args_code_array)
}

fn generate_new(
    fields: &Punctuated<Field, Comma>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let args_code = fields.iter().enumerate().map(move |(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if field_ident == "_reserved" {
            return quote! {};
        }
        if i < fields.len() - 1 {
            quote! { #field_ident: #field_type, }
        } else {
            quote! { #field_ident: #field_type }
        }
    });
    let struct_code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            let reserved_len = match &field.ty {
                Type::Array(arr) => &arr.len,
                _ => panic!("_reserved can only be an array"),
            };
            return quote! { _reserved: [0; #reserved_len], };
        }
        return quote! { #field_ident, };
    });
    (args_code, struct_code)
}

fn generate_add(
    fields: &Punctuated<Field, Comma>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_add(other.#field_ident[i])?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_add(other.#field_ident)?, },
        }
    });

    (code, code_array)
}

fn generate_add_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! {
                for i in 0..self.#field_ident.len() {
                    self.#field_ident[i] = self.#field_ident[i].checked_add(other.#field_ident[i])?;
                }
            },
            _ => quote! { self.#field_ident = self.#field_ident.checked_add(other.#field_ident)?; },
        }
    });
    code
}

fn generate_sub(
    fields: &Punctuated<Field, Comma>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_sub(other.#field_ident[i])?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_sub(other.#field_ident)?, },
        }
    });

    (code, code_array)
}

fn generate_sub_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! {
                for i in 0..self.#field_ident.len() {
                    self.#field_ident[i] = self.#field_ident[i].checked_sub(other.#field_ident[i])?;
                }
            },
            _ => quote! { self.#field_ident = self.#field_ident.checked_sub(other.#field_ident)?; },
        }
    });
    code
}

fn generate_mul(
    fields: &Punctuated<Field, Comma>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_mul(other.#field_ident[i])?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_mul(other.#field_ident)?, },
        }
    });

    (code, code_array)
}

fn generate_div(
    fields: &Punctuated<Field, Comma>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_div(other.#field_ident[i])?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_div(other.#field_ident)?, },
        }
    });

    (code, code_array)
}

fn generate_div_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let factor2 = factor.clone();
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_div(#factor.into())?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_div(#factor2)?, },
        }
    });

    (code, code_array)
}

fn generate_mul_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let factor2 = factor.clone();
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = self.#field_ident[i].checked_mul(#factor.into())?;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { self.#field_ident.checked_mul(#factor2)?, },
        }
    });

    (code, code_array)
}

fn generate_mul_fraction(
    fields: &Punctuated<Field, Comma>,
    numerator: Ident,
    denominator: Ident,
    fields_type: Ident,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let numerator2 = numerator.clone();
    let denominator2 = denominator.clone();
    let code_array = fields
        .into_iter()
        .enumerate()
        .filter(|(_i, field)| match &field.ty {
            Type::Array(_arr) => field.ident.as_ref().unwrap() != "_reserved",
            _ => false,
        })
        .map(move |(_i, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let (field_size, field_type) = match &field.ty {
                Type::Array(arr) => (&arr.len, &arr.elem),
                _ => panic!("Only arrays are accepted"),
            };
            quote! {
                let mut #field_ident = [#field_type::default(); #field_size];
                for i in 0..self.#field_ident.len() {
                    #field_ident[i] = ((self.#field_ident[i] as u128).checked_mul(#numerator as u128)?.checked_div(#denominator as u128)?) as #field_type;
                }
            }
        });
    let code = fields.into_iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        if field_ident == "_reserved" {
            return quote! {};
        }
        match &field.ty {
            Type::Array(_arr) => quote! { #field_ident, },
            _ => quote! { ((self.#field_ident as u128).checked_mul(#numerator as u128)?.checked_div(#denominator as u128)?) as #fields_type, },
        }
    });

    (code, code_array)
}
