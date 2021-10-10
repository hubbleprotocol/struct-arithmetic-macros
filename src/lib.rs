use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput,
    Field, Fields, Ident, Path, Type, TypePath,
};

#[proc_macro_derive(StructArithmetic)]
pub fn struct_arithmetic(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
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

    // println!("Fields {:?}", fields);
    // println!("Type {:?}", field_type);

    let factor = Ident::new("factor", Span::call_site());
    let factor_type = Ident::new(&field_type, Span::call_site());

    let addition = generate_add(&fields);
    let addition_assign = generate_add_assign(&fields);
    let subtraction = generate_sub(&fields);
    let subtraction_assign = generate_sub_assign(&fields);
    let multiplication = generate_mul(&fields);
    let division = generate_div(&fields);
    let division_scalar = generate_div_scalar(&fields, factor.clone());
    let multiplication_scalar = generate_mul_scalar(&fields, factor.clone());

    let modified = quote! {
        impl #name {
            pub fn add(&self, other: #name) -> #name {
                #name {
                #(#addition)*
                }
            }

            pub fn add_assign(&mut self, other: #name) {
                #(#addition_assign)*
            }

            pub fn sub(&self, other: #name) -> #name {
                #name {
                #(#subtraction)*
                }
            }

            pub fn sub_assign(&mut self, other: #name) {
                #(#subtraction_assign)*
            }

            pub fn mul(&self, other: #name) -> #name {
                #name {
                #(#multiplication)*
                }
            }

            pub fn div(&self, other: #name) -> #name {
                #name {
                #(#division)*
                }
            }

            pub fn div_scalar(&self, factor: #factor_type) -> #name {
                #name {
                #(#division_scalar)*
                }
            }

            pub fn mul_scalar(&self, factor: #factor_type) -> #name {
                #name {
                #(#multiplication_scalar)*
                }
            }

        }
    };
    TokenStream::from(modified)
}

fn generate_add(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_add(other.#field_ident).unwrap(), }
    });
    code
}

fn generate_add_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident = self.#field_ident.checked_sub(other.#field_ident).unwrap(); }
    });
    code
}

fn generate_sub(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_sub(other.#field_ident).unwrap(), }
    });
    code
}

fn generate_sub_assign(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { self.#field_ident = self.#field_ident.checked_add(other.#field_ident).unwrap(); }
    });
    code
}

fn generate_mul(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_mul(other.#field_ident).unwrap(), }
    });
    code
}

fn generate_div(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_div(other.#field_ident).unwrap(), }
    });
    code
}

fn generate_div_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_div(#factor).unwrap(), }
    });
    code
}

fn generate_mul_scalar(
    fields: &Punctuated<Field, Comma>,
    factor: Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let code = fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: self.#field_ident.checked_mul(#factor).unwrap(), }
    });
    code
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        // let x = Fields [
        //     Field {
        //         attrs: [],
        //         vis: Public(VisPublic { pub_token: Pub }),
        //         ident: Some(Ident { ident: "sol", span: #0 bytes(91..94) }),
        //         colon_token: Some(Colon),
        //         ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "u64", span: #0 bytes(96..99) }, arguments: None }] } }) }, Comma,
        //     Field { attrs: [], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident { ident: "eth", span: #0 bytes(109..112) }), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "u64", span: #0 bytes(114..117) }, arguments: None }] } }) }, Comma,
        //     Field { attrs: [], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident { ident: "btc", span: #0 bytes(127..130) }), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "u64", span: #0 bytes(132..135) }, arguments: None }] } }) }, Comma]
    }
}
