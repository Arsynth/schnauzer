use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, token::Comma};

#[proc_macro_derive(AutoEnumFields)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse_macro_input!(input);

    impl_auto_enum_fields(&ast).into()
}

fn impl_auto_enum_fields(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let id = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(f) => fields_token_stream(&f.named),
            syn::Fields::Unnamed(f) => fields_token_stream(&f.unnamed),
            syn::Fields::Unit => panic!("Unit structs is not supported by derive"),
        },
        syn::Data::Enum(data) => enum_cases_token_stream(id, data),
        _ => panic!("Fields enumeration derive can only implemented for structs and enums"),
    };

    let ident = &ast.ident;

    let output = quote! {
        impl AutoEnumFields for #ident {
            fn all_fields(&self) -> Vec<Field> {
                let mut v: Vec<Field> = Vec::new();
                #fields;
                v
            }
        }
    };

    output.into()
}

fn enum_cases_token_stream(
    ident: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let matches: Vec<_> = data_enum
        .variants
        .iter()
        .map(|var| arm_tokens(var, ident))
        .collect();

    quote! {
        match self {
            #(#matches)*
        };
    }
}

fn arm_tokens(var: &syn::Variant, enum_id: &syn::Ident) -> proc_macro2::TokenStream {
    let var_id = &var.ident;
    let fields_tokens = match &var.fields {
        syn::Fields::Named(f) => case_fields_tokens(&f.named),
        syn::Fields::Unnamed(f) => case_fields_tokens(&f.unnamed),
        syn::Fields::Unit => Vec::new(),
    };

    if fields_tokens.len() == 0 {
        return quote! {
            #enum_id::#var_id => {

            }
        };
    }

    let inserts_tokens: Vec<_> = fields_tokens
        .iter()
        .map(|field| {
            quote! {
                {
                    let mut tmp = #field.all_fields();
                    v.append(&mut tmp);
                }
            }
        })
        .collect();

    quote! {
        #enum_id::#var_id(#(#fields_tokens),*) => {
            #(#inserts_tokens)*
        }
    }
}

fn case_fields_tokens(
    fields: &syn::punctuated::Punctuated<syn::Field, Comma>,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let ident = &field.ident.as_ref().map(|i| quote! {#i}).unwrap_or({
                let new_name = format!("field{idx}");
                let new_id = proc_macro2::Ident::new(&new_name, proc_macro2::Span::call_site());
                quote! {#new_id}
            });
            quote!(#ident)
        })
        .collect()
}

fn fields_token_stream(
    fields: &syn::punctuated::Punctuated<syn::Field, Comma>,
) -> proc_macro2::TokenStream {
    let v: Vec<_> = fields
        .iter()
        .filter(|field| match &field.vis {
            syn::Visibility::Public(_) => true,
            _ => false,
        })
        .enumerate()
        .map(|(idx, field)| {
            let ident = &field.ident.as_ref().map(|i| quote! {#i}).unwrap_or({
                let t = proc_macro2::Literal::usize_unsuffixed(idx);
                quote! {#t}
            });
            let value = quote! {
                format!("{:?}", self.#ident)
            };
            quote! {
                v.push(Field::new(stringify!(#ident).to_string(), #value));
            }
        })
        .collect();

    quote! {
        #(#v)*;
    }
}
