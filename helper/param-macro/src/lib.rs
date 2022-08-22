extern crate proc_macro;

use convert_case::{Case, Casing};
use darling::{ast, util, FromDeriveInput, FromField, FromTypeParam};
use quote::quote;

#[derive(FromDeriveInput)]
#[darling(attributes(params), supports(struct_named))]
struct Params {
    pub data: ast::Data<util::Ignored, ParamField>,
    pub generics: ast::Generics<ast::GenericParam<GenericParams>>,
    #[darling(default)]
    pub rename_all: Option<syn::LitStr>,
}

#[derive(FromTypeParam)]
#[darling(attributes(params))]
struct GenericParams {
    pub ident: syn::Ident,
}

#[derive(FromField)]
#[darling(attributes(params), forward_attrs(allow, doc, cfg))]
struct ParamField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    #[darling(default)]
    pub rename: Option<syn::LitStr>,
}

#[proc_macro_derive(SerializeParams, attributes(params))]
pub fn derive_serialize_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let struct_name = &input.ident;
    let params = Params::from_derive_input(&input).unwrap();
    let impl_def = match params.generics.params.len() {
        0 => quote! {
            impl #struct_name
        },
        _ => {
            let mut type_idents = Vec::new();
            let idents = params
                .generics
                .params
                .iter()
                .map(|param| match param {
                    ast::GenericParam::Type(param) => {
                        let ident = param.ident.clone();
                        type_idents.push(ident.clone());
                        (quote! {#ident}, quote! {#ident})
                    }
                    ast::GenericParam::Lifetime(param) => {
                        let ident = param.lifetime.clone();
                        (quote! {#ident}, quote! {#ident})
                    }
                    ast::GenericParam::Const(param) => {
                        let ident = param.ident.clone();
                        let ty = param.ty.clone();
                        (quote! {const #ident: #ty}, quote! {#ident})
                    }
                })
                .collect::<Vec<_>>();
            let impl_generics = idents.iter().map(|(ident, _)| ident.clone());
            let ty_generics = idents.iter().map(|(_, ident)| ident.clone());
            quote! {
                impl<#(#impl_generics),*> #struct_name<#(#ty_generics),*>
                where #(#type_idents: std::fmt::Display),*
            }
        }
    };
    let rename_method = match params.rename_all {
        Some(lit) => {
            let lit = lit.value();
            match lit.as_str() {
                "snake_case" => |s: &str| s.to_case(Case::Snake),
                "kebab-case" => |s: &str| s.to_case(Case::Kebab),
                "camelCase" => |s: &str| s.to_case(Case::Camel),
                "PascalCase" => |s: &str| s.to_case(Case::Pascal),
                "SCREAMING_SNAKE_CASE" => |s: &str| s.to_case(Case::ScreamingSnake),
                "Train-Case" => |s: &str| s.to_case(Case::Train),
                _ => panic!("Unknown rename_all value: {}", lit),
            }
        }
        None => |s: &str| s.to_string(),
    };

    // Get struct fields
    let fields = match params.data {
        ast::Data::Struct(ref data) => data,
        _ => panic!("#[derive(SerializeParams)] only works on structs"),
    };

    let field_idents = fields
        .fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect::<Vec<_>>();

    let field_idents_str = fields
        .fields
        .iter()
        .map(|field| {
            if let Some(lit) = &field.rename {
                lit.value()
            } else {
                let ident = field.ident.as_ref().unwrap().to_string();
                rename_method(&ident)
            }
        })
        .collect::<Vec<_>>();

    // Check Option<> fields
    let option_field_flags = fields
        .fields
        .iter()
        .map(|field| match field.ty {
            syn::Type::Path(ref path) => {
                matches!(
                    path.path
                        .segments
                        .last()
                        .unwrap()
                        .ident
                        .to_string()
                        .as_str(),
                    "Option"
                )
            }
            _ => false,
        })
        .collect::<Vec<_>>();

    let fields_impl = field_idents
        .iter()
        .zip(option_field_flags.iter().zip(field_idents_str))
        .map(|(ident, (flag, ident_str))| {
            if *flag {
                quote! {
                    if let Some(ref v) = self.#ident {
                        params.push((#ident_str.to_string(), v.to_string()));
                    }
                }
            } else {
                quote! {
                    params.push((#ident_str.to_string(), self.#ident.to_string()));
                }
            }
        })
        .collect::<Vec<_>>();

    // Generate impl block
    let gen = quote! {
        #impl_def {
            fn serialize_params(&self) -> Vec<(String, String)> {
                let mut params = Vec::new();
                #(
                    #fields_impl
                )*
                params
            }
        }
    };
    gen.into()
}
