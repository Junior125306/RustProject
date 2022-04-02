use proc_macro2::{Ident, TokenStream};
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
};

pub struct BuilderContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };
        Self { name, fields }
    }

    pub fn generate(self) -> TokenStream {
        // builder name
        let builder_name = todo!();
        let optionized_fields = vec![];
        let methods = vec![];
        let assigns = vec![];
        let ast = quote! {
            #[derive(Debug,Default)]
            struct #builder_name {
                #(#optionized_fields,)*
            }
            impl #builder_name {
                #(#methods)*
            }
            pub fn finish(mut self)-> Result<#name,&'static str>{
                Ok(#name{
                    #(#assings,)*
                })
            }
            impl #name {
                fn builder()->#builder_name {
                    Default::default()
                }
            }
        };
    }
}
