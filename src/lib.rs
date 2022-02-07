use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    self, parse_macro_input,
    punctuated::{Pair, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Expr, Field, Fields, Ident, Path, Type, TypePath,
};

#[proc_macro_derive(StructArithmetic, attributes(helper))]
pub fn struct_arithmetic(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let mut fields: Punctuated<Field, Comma> = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with named fields can be annotated with ToUrl"),
    };
    let (has_reserved, reserved_size) =
        if fields.last().unwrap().ident.as_ref().unwrap() == "_reserved" {
            match fields.pop().unwrap() {
                Pair::Punctuated(field, _) => match field.ty {
                    Type::Array(arr) => (true, Some(arr.len)),
                    _ => panic!("Only arrays are accepted as _reserved"),
                },
                _ => panic!("END token not accepted as _reserved"),
            }
        } else {
            (false, None)
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

    // println!("Fields {:?}", fields);
    // println!("Type {:?}", field_type);

    let factor = Ident::new("factor", Span::call_site());
    let numerator = Ident::new("numerator", Span::call_site());
    let denominator = Ident::new("denominator", Span::call_site());
    let fields_type = Ident::new(&field_type, Span::call_site());
    // let token_type = Ident::new("token", Span::call_site());

    let addition = generate_add(&fields);
    let addition_assign = generate_add_assign(&fields);
    let subtraction = generate_sub(&fields);
    let subtraction_assign = generate_sub_assign(&fields);
    let multiplication = generate_mul(&fields);
    let division = generate_div(&fields);
    let division_scalar = generate_div_scalar(&fields, factor.clone());
    let multiplication_scalar = generate_mul_scalar(&fields, factor.clone());
    let multiplication_fraction =
        generate_mul_fraction(&fields, numerator, denominator, fields_type.clone());

    let (new_constructor_args, new_constructor_struct) =
        generate_new(&fields, fields_type.clone(), has_reserved, reserved_size);
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
            }

            pub fn add(&self, other: &#name) -> Option<#name> {
                Some(#name::new(
                    #(#addition)*
                ))
            }

            pub fn add_assign(&mut self, other: &#name) -> Option<()> {
                #(#addition_assign)*

                Some(())
            }

            pub fn sub(&self, other: &#name) -> Option<#name> {
                Some(#name::new(
                    #(#subtraction)*
                ))
            }

            pub fn sub_assign(&mut self, other: &#name) -> Option<()> {
                #(#subtraction_assign)*

                Some(())
            }

            pub fn div(&self, other: &#name) -> Option<#name> {
                Some(#name::new(
                    #(#division)*
                ))
            }

            pub fn div_scalar(&self, factor: #fields_type) -> Option<#name> {
                Some(#name::new(
                    #(#division_scalar)*
                ))
            }

            pub fn mul(&self, other: &#name) -> Option<#name> {
                Some(#name::new(
                    #(#multiplication)*
                ))
            }

            pub fn mul_scalar(&self, factor: #fields_type) -> Option<#name> {
                Some(#name::new(
                    #(#multiplication_scalar)*
                ))
            }

            pub fn mul_fraction(&self, numerator: #fields_type, denominator: #fields_type) -> Option<#name> {
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

// fn generate_token_amount(
//     fields: &Punctuated<Field, Comma>,
//     enum_type: Ident,
// ) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
//     let args_code = fields.iter().enumerate().map(move |(_i, field)| {
//         let enum_variant = Ident::new(&stringify!(&field).to_uppercase(), Span::call_site());
//         let field_ident = field.ident.as_ref().unwrap();
//         quote! { enum_type::#enum_variant => self.#field_ident,  }
//     });

//     args_code
// }

fn generate_is_zero(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let args_code = fields.iter().enumerate().map(move |(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        if i < fields.len() - 1 {
            quote! { self.#field_ident == 0 && }
        } else {
            quote! { self.#field_ident == 0  }
        }
    });

    args_code
}

fn generate_new(
    fields: &Punctuated<Field, Comma>,
    factor_type: Ident,
    has_reserved: bool,
    reserved_size: Option<Expr>,
) -> (
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
    impl Iterator<Item = proc_macro2::TokenStream> + '_,
) {
    let args_code = fields.iter().enumerate().map(move |(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        if i < fields.len() - 1 {
            quote! { #field_ident: #factor_type, }
        } else {
            quote! { #field_ident: #factor_type }
        }
    });
    let struct_code = fields.iter().enumerate().map(move |(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        if i < fields.len() - 1 {
            quote! { #field_ident, }
        } else {
            match has_reserved {
                false => quote! { #field_ident },
                true => quote! {#field_ident, _reserved: [0; #reserved_size]},
            }
        }
    });
    (args_code, struct_code)
}

fn generate_add(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_add(other.#field_ident)?, }
    });
    code
}

fn generate_add_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident = self.#field_ident.checked_add(other.#field_ident)?; }
    });
    code
}

fn generate_sub(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_sub(other.#field_ident)?, }
    });
    code
}

fn generate_sub_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident = self.#field_ident.checked_sub(other.#field_ident)?; }
    });
    code
}

fn generate_mul(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_mul(other.#field_ident)?, }
    });
    code
}

fn generate_div(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_div(other.#field_ident)?, }
    });
    code
}

fn generate_div_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_div(#factor)?, }
    });
    code
}

fn generate_mul_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident.checked_mul(#factor)?, }
    });
    code
}

fn generate_mul_fraction(
    fields: &Punctuated<Field, Comma>,
    numerator: Ident,
    denominator: Ident,
    fields_type: Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { ((self.#field_ident as u128).checked_mul(#numerator as u128)?.checked_div(#denominator as u128)?) as #fields_type, }
    });
    code
}
