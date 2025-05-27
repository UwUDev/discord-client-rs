use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

// TODO: Macro for types like AuthenticatorType

fn is_u64_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u64" && segment.arguments.is_empty();
        }
    }
    false
}

fn is_option_u64_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(inner_type) = &args.args[0] {
                            return is_u64_type(inner_type);
                        }
                    }
                }
            }
        }
    }
    false
}

fn generate_created_at_impl(input: DeriveInput, is_option: bool) -> TokenStream {
    let struct_name = &input.ident;

    let id_field = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .find(|f| f.ident.as_ref().map_or(false, |i| i == "id"))
                .map(|f| &f.ty),
            _ => None,
        },
        _ => None,
    };

    let output = match id_field {
        Some(ty) => {
            let is_valid_type = if is_option {
                is_option_u64_type(ty)
            } else {
                is_u64_type(ty)
            };

            if !is_valid_type {
                let expected_type = if is_option { "Option<u64>" } else { "u64" };
                return syn::Error::new_spanned(
                    ty,
                    format!("Field 'id' must be of type {}", expected_type),
                )
                .to_compile_error()
                .into();
            }

            if is_option {
                quote! {
                    impl #struct_name {
                        pub fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
                            self.id.map(|id| {
                                let timestamp = (id >> 22) + 1420070400000;
                                <chrono::Utc as chrono::TimeZone>::timestamp_millis_opt(&chrono::Utc, timestamp as i64)
                                    .single()
                                    .unwrap()
                            })
                        }
                    }
                }
            } else {
                quote! {
                    impl #struct_name {
                        pub fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
                            let timestamp = (self.id >> 22) + 1420070400000;
                            <chrono::Utc as chrono::TimeZone>::timestamp_millis_opt(&chrono::Utc, timestamp as i64).single()
                        }
                    }
                }
            }
        }
        None => syn::Error::new(
            struct_name.span(),
            format!(
                "Struct must have field 'id' of type {}",
                if is_option { "Option<u64>" } else { "u64" }
            ),
        )
        .to_compile_error(),
    };

    output.into()
}

#[proc_macro_derive(CreatedAt)]
pub fn derive_created_at(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_created_at_impl(input, false)
}

#[proc_macro_derive(OptionCreatedAt)]
pub fn derive_option_created_at(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_created_at_impl(input, true)
}

#[proc_macro_derive(EnumFromPrimitive)]
pub fn derive_enum_from_primitive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFromPrimitive ne peut être utilisé que sur des enums"),
    };

    let match_arms = variants.iter().filter_map(|v| {
        if v.fields.is_empty() {
            let variant_name = &v.ident;
            let discriminant = v.discriminant.as_ref().and_then(|(_, expr)| {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit), .. }) = expr {
                    Some(lit.base10_parse::<u8>().unwrap())
                } else {
                    None
                }
            });

            discriminant.map(|d| {
                quote! { #d => Ok(#enum_name::#variant_name), }
            })
        } else {
            None
        }
    });

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u8::deserialize(deserializer)?;
                match value {
                    #(#match_arms)*
                    n => Ok(#enum_name::UNKNOWN(n)),
                }
            }
        }
    };

    TokenStream::from(expanded)
}