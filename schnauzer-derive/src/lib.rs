use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro_derive(AutoEnumFields)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input);

    impl_auto_enum_fields(&ast).into()
}

fn impl_auto_enum_fields(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(f) => &f.named,
            syn::Fields::Unnamed(f) => &f.unnamed,
            syn::Fields::Unit => panic!("Unit structs is not supported by derive"),
        },
        _ => panic!("Fields enumeration derive can only implemented for structs"),
    };

    let q: Vec<_> = fields
    .iter()
    .filter(|field| {
        match &field.vis {
            syn::Visibility::Public(_) => true,
            _ => false,
        }
    })
    .enumerate()
    .map(|(idx, field)| {
        let ident = &field.ident.as_ref().map(|i| quote!{#i}).unwrap_or({let t = proc_macro2::Literal::usize_unsuffixed(idx); quote!{#t}});
        let value = quote! {
            format!("{:?}", self.#ident)
        };
        quote! {
            v.push(Field::new(stringify!(#ident).to_string(), #value));
        }
    }).collect();

    let ident = &ast.ident;

    let output = quote! {
        impl AutoEnumFields for #ident {
            fn all_fields(&self) -> Vec<Field> {
                let mut v: Vec<Field> = Vec::new();
                #(#q)*;
                v
            }
        }
    };

    output.into()
}