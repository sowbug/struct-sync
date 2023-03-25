// Copyright (c) 2023 Mike Tsao. All rights reserved.

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Generics};

#[proc_macro_derive(Synchronization, attributes(sync))]
pub fn synchronization_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let enum_name = format_ident!("{}Message", struct_name);
    TokenStream::from(parse_synchronization_data(
        &struct_name,
        &input.generics,
        &enum_name,
        &input.data,
    ))
}

fn parse_synchronization_data(
    struct_name: &Ident,
    generics: &Generics,
    enum_name: &Ident,
    data: &Data,
) -> proc_macro2::TokenStream {
    let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
    let mut enum_set_method_names = Vec::default();
    let mut enum_set_method_original_names = Vec::default();
    let mut enum_snake_names = Vec::default();
    let mut enum_variant_names = Vec::default();
    let mut enum_variant_fields = Vec::default();

    // Code adapted from https://blog.turbo.fish/proc-macro-error-handling/
    // Thank you!
    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };
    let sync_fields = fields.into_iter().fold(Vec::default(), |mut v, f| {
        let attrs: Vec<_> = f
            .attrs
            .iter()
            .filter(|attr| attr.path.is_ident("sync"))
            .collect();
        if !attrs.is_empty() {
            match &f.ty {
                syn::Type::Path(t) => {
                    if let Some(ident) = t.path.get_ident() {
                        v.push((f.ident.as_ref().unwrap().clone(), ident.clone()));
                    }
                }
                _ => todo!(),
            }
        }
        v
    });

    for (field_name, field_type) in sync_fields {
        enum_set_method_names.push(format_ident!(
            "set_and_propagate_{}",
            field_name.to_string(),
        ));
        enum_set_method_original_names.push(format_ident!("set_{}", field_name.to_string(),));
        enum_snake_names.push(format_ident!("{}", field_name.to_string(),));
        enum_variant_names.push(format_ident!(
            "{}",
            field_name.to_string().to_case(Case::Pascal),
        ));
        enum_variant_fields.push(format_ident!("{}", field_type));
    }

    let enum_block = quote! {
        #[derive(Clone, Display, Debug, EnumCountMacro, EnumString, FromRepr, IntoStaticStr)]
        #[strum(serialize_all = "kebab-case")]
        pub enum #enum_name {
            #struct_name ( #struct_name ),
            #( #enum_variant_names ( #enum_variant_fields ) ),*
        }
    };
    let impl_block = quote! {
        impl #generics #struct_name #ty_generics {
            pub fn update(&mut self, message: #enum_name) {
                match message {
                    #enum_name::#struct_name(v) => *self = v,
                    #( #enum_name::#enum_variant_names(v) => self.#enum_set_method_original_names(v) ),*
                }
            }

            pub fn message_for(
                &self,
                param_name: &str,
                value: groove_core::control::F32ControlValue,
            ) -> Option<#enum_name> {
                if let Ok(message) = #enum_name::from_str(param_name) {
                    match message {
                        #enum_name::#struct_name(_) => {}
                        #( #enum_name::#enum_variant_names(_) => {return Some(#enum_name::#enum_variant_names(value.into()));} )*
                    }
                }
                None
            }

        }
    };
    let controllable_block = quote! {
        impl groove_core::traits::Controllable for #generics #struct_name #ty_generics {
            fn control_name_for_index(&self, index: usize) -> Option<&'static str> {
                if let Some(message) = #enum_name::from_repr(index + 1) {
                    Some(message.into())
                } else {
                    None
                }
            }
            fn control_index_count(&self) -> usize {
                #enum_name::COUNT - 1
            }
        }
    };
    quote! {
        #[automatically_derived]
        #enum_block
        #[automatically_derived]
        #impl_block
        #[automatically_derived]
        #controllable_block
    }
}
