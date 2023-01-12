extern crate proc_macro;

use darling::{ast, util, FromDeriveInput, FromField, FromTypeParam};
use proc_macro2::Ident;
use quote::quote;
use syn::LitStr;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(attr), supports(struct_named))]
struct Attr {
    pub data: ast::Data<util::Ignored, AttrField>,
    pub generics: ast::Generics<ast::GenericParam<GenericParams>>,
}

#[derive(Debug, FromTypeParam)]
#[darling(attributes(attr))]
struct GenericParams {
    pub ident: syn::Ident,
}

#[derive(Debug, FromField)]
#[darling(attributes(attr), forward_attrs(allow, doc, cfg))]
struct AttrField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    #[darling(default)]
    pub default: Option<syn::LitStr>,
    #[darling(default)]
    pub min: Option<syn::LitStr>,
    #[darling(default)]
    pub max: Option<syn::LitStr>,

    #[darling(default)]
    pub parse: Option<syn::LitStr>,
}

#[proc_macro_derive(AppWriteModel, attributes(attr))]
pub fn convert_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let struct_name = &input.ident;
    let attr = Attr::from_derive_input(&input).unwrap();
    let impl_def = get_impl_def(struct_name, &attr);

    let fields = match attr.data {
        ast::Data::Struct(ref data) => data,
        _ => panic!("#[derive(Attribute)] only works on structs"),
    };
    let mut attrs = vec![];

    for field in fields.fields.iter() {
        let attr = get_attr(field);
        attrs.push(attr);
    }

    let gen = quote! {
        #impl_def {
            pub fn get_attribute_definitions() -> Vec<::appwrite::models::attribute::Attribute> {
                let mut attrs_list = vec![];
                #(#attrs)*
                attrs_list
            }

            pub async fn create_attribute(client: &::appwrite::prelude::AppWriteClient, collection: &::appwrite::prelude::Collection,) -> Result<(), appwrite::error::Error> {
                let database_id = &collection.database_id;
                let collection_id = &collection.id;
                let attributes = Self::get_attribute_definitions();
                for attribute in attributes {
                    ::appwrite::prelude::DatabasesService::create_attribute(
                        client,
                        database_id,
                        collection_id,
                        attribute
                    ).await?;
                }
                Ok(())
            }

            pub async fn create_document(
                &self,
                client: &::appwrite::prelude::AppWriteClient,
                collection: &::appwrite::prelude::Collection,
                document_id: ::appwrite::prelude::DocumentId,
                permissions: Vec<::appwrite::prelude::Permission>,
            ) -> Result<::appwrite::prelude::Document<Self>, appwrite::error::Error>
            where Self: ::serde::Serialize + for<'de> serde::Deserialize<'de>
            {
                let database_id = &collection.database_id;
                let collection_id = &collection.id;
                let document = collection.create_document::<Self>(
                    client,
                    ::appwrite::prelude::CreateDocumentPayload {
                        data: ::serde_json::json! { self },
                        document_id,
                        permissions,
                    }
                ).await?;
                Ok(document)
            }
        }
    };
    gen.into()
}

fn get_attr(field: &AttrField) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let mut ty = field.ty.clone();

    let is_option = match &ty {
        syn::Type::Path(path) => {
            let path = &path.path;
            let segment = path.segments.first();
            match segment {
                Some(segment) => {
                    let ident = &segment.ident;
                    ident == "Option"
                }
                _ => false,
            }
        }
        _ => false,
    };
    if is_option {
        let path = match &ty {
            syn::Type::Path(path) => &path.path,
            _ => panic!(""),
        };
        let segment = path.segments.first();
        ty = match segment {
            Some(segment) => {
                let arguments = &segment.arguments;
                match arguments {
                    syn::PathArguments::AngleBracketed(arguments) => {
                        let args = &arguments.args;
                        let arg = args.first();
                        match arg {
                            Some(syn::GenericArgument::Type(ty)) => ty.clone(),
                            _ => panic!(""),
                        }
                    }
                    _ => panic!(""),
                }
            }
            _ => panic!(""),
        };
    }

    let is_vector = match &ty {
        syn::Type::Path(path) => {
            let path = &path.path;
            let segment = path.segments.first();
            match segment {
                Some(segment) => {
                    let ident = &segment.ident;
                    ident == "Vec"
                }
                _ => false,
            }
        }
        _ => false,
    };
    if is_vector {
        let path = match &ty {
            syn::Type::Path(path) => &path.path,
            _ => panic!(""),
        };
        let segment = path.segments.first();
        ty = match segment {
            Some(segment) => {
                let arguments = &segment.arguments;
                match arguments {
                    syn::PathArguments::AngleBracketed(arguments) => {
                        let args = &arguments.args;
                        let arg = args.first();
                        match arg {
                            Some(syn::GenericArgument::Type(ty)) => ty.clone(),
                            _ => panic!(""),
                        }
                    }
                    _ => panic!(""),
                }
            }
            _ => panic!(""),
        };
    }
    let default = field.default.as_ref();
    let min = field.min.as_ref();
    let max = field.max.as_ref();
    let required = !is_option;
    let type_name = match &ty {
        syn::Type::Path(path) => {
            let path = &path.path;
            let segment = path.segments.first();
            match segment {
                Some(segment) => {
                    let ident = &segment.ident;
                    ident.to_string()
                }
                _ => panic!(""),
            }
        }
        _ => panic!(""),
    };
    let attr = match type_name.as_str() {
        "i64" | "i32" | "i16" | "i8" | "u64" | "u32" | "u16" | "u8" => {
            let vals = parse_values::<i64>(default, min, max);
            let (default, min, max) = (&vals[0], &vals[1], &vals[2]);
            let ident = ident.to_string();
            quote! {
                attrs_list.push(::appwrite::models::attribute::Attribute::new_integer(
                    #ident, #required, #default, #min, #max, Some(#is_vector)
                ));
            }
        }
        "bool" => {
            let vals = parse_values::<bool>(default, min, max);
            let (default, _, _) = (&vals[0], &vals[1], &vals[2]);
            let ident = ident.to_string();
            quote! {
                attrs_list.push(::appwrite::models::attribute::Attribute::new_boolean(
                    #ident, #required, #default, Some(#is_vector)
                ));
            }
        }
        "f32" | "f64" => {
            let vals = parse_values::<bool>(default, min, max);
            let (default, min, max) = (&vals[0], &vals[1], &vals[2]);
            let ident = ident.to_string();
            quote! {
                attrs_list.push(::appwrite::models::attribute::Attribute::new_double(
                    #ident, #required, #default, #min, #max, Some(#is_vector)
                ));
            }
        }
        "String" => {
            let ident = ident.to_string();
            let parse_type = field.parse.as_ref();
            let parse_type = match parse_type {
                Some(lit) => {
                    let ty = lit.value();
                    match ty.as_str() {
                        "Ip" | "Url" | "Email" | "DateTime" => ty,
                        _ => "Str".into(),
                    }
                }
                _ => "Str".into(),
            };
            let default = &to_quote(default);
            if parse_type == "Ip" {
                return quote! {
                    attrs_list.push(::appwrite::models::attribute::Attribute::new_ip(
                        #ident, #required, #default, Some(#is_vector)
                    ));
                };
            }
            if parse_type == "Url" {
                return quote! {
                    attrs_list.push(::appwrite::models::attribute::Attribute::new_url(
                        #ident, #required, #default, Some(#is_vector)
                    ));
                };
            }
            if parse_type == "Email" {
                return quote! {
                    attrs_list.push(::appwrite::models::attribute::Attribute::new_email(
                        #ident, #required, #default, Some(#is_vector)
                    ));
                };
            }
            if parse_type == "DateTime" {
                return quote! {
                    attrs_list.push(::appwrite::models::attribute::Attribute::new_datetime(
                        #ident, #required, #default, Some(#is_vector)
                    ));
                };
            }
            let max = field.max.as_ref();
            let max = match max {
                Some(lit) => {
                    let v = lit.value();
                    v.parse::<usize>().unwrap_or(1024)
                }
                _ => 1024,
            };
            quote! {
                attrs_list.push(::appwrite::models::attribute::Attribute::new_string(
                    #ident, #required, #default, #max, Some(#is_vector)
                ));
            }
        }
        _ => quote! {},
    };
    attr
}

fn to_quote(default: Option<&LitStr>) -> proc_macro2::TokenStream {
    match default {
        Some(v) => {
            let v = v.value();
            quote! { Some(#v) }
        }
        _ => quote! { None },
    }
}

fn parse_values<T>(
    default: Option<&LitStr>,
    min: Option<&LitStr>,
    max: Option<&LitStr>,
) -> Vec<proc_macro2::TokenStream>
where
    T: std::str::FromStr + darling::ToTokens,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let default = match default {
        Some(v) => {
            let v = v.value();
            let v = v.parse::<T>().unwrap();
            quote! { Some(#v) }
        }
        _ => quote! { None },
    };
    let min = match min {
        Some(v) => {
            let v = v.value();
            let v = v.parse::<T>().unwrap();
            quote! { Some(#v) }
        }
        _ => quote! { None },
    };
    let max = match max {
        Some(v) => {
            let v = v.value();
            let v = v.parse::<T>().unwrap();
            quote! { Some(#v) }
        }
        _ => quote! { None },
    };
    vec![default, min, max]
}

fn get_impl_def(struct_name: &Ident, attr: &Attr) -> proc_macro2::TokenStream {
    match attr.generics.params.len() {
        0 => quote! {
            impl #struct_name
        },
        _ => {
            let mut type_idents = Vec::new();
            let idents = attr
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
    }
}
