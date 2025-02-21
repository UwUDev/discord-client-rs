use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

// discord_client_macros/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_derive(CreatedAt)]
pub fn derive_created_at(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let id_field = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter()
                .find(|f| f.ident.as_ref().map_or(false, |i| i == "id"))
                .map(|f| &f.ty),
            _ => None,
        },
        _ => None,
    };


    let output = match id_field {
        Some(ty) => {
            if quote!(#ty).to_string() != "u64" {
                return syn::Error::new_spanned(ty, "Field 'id' must be of type u64")
                    .to_compile_error()
                    .into();
            }

            quote! {
                use chrono::TimeZone;

                impl #struct_name {
                    pub fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
                        let timestamp = (self.id >> 22) + 1420070400000;
                        chrono::Utc.timestamp_millis_opt(timestamp as i64).single()
                    }
                }
            }
        }
        None => syn::Error::new(struct_name.span(), "Struct must have field 'id' of type u64")
            .to_compile_error(),
    };

    output.into()
}