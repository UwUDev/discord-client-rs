use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};
use convert_case::{Case, Casing};


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

#[proc_macro_derive(EnumFromPrimitive, attributes(default))]
pub fn derive_enum_from_primitive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("EnumFromPrimitive can only be used on enums"),
    };

    let mut match_arms = Vec::new();
    let mut as_u8_arms = Vec::new();
    let mut default_variant = None;
    let mut has_unknown = false;

    for variant in &variants {
        let var_name = &variant.ident;

        if var_name == "Unknown" {
            if let syn::Fields::Unnamed(fields) = &variant.fields {
                if fields.unnamed.len() == 1 {
                    has_unknown = true;
                    continue;
                }
            }
            panic!("The Unknown variant must be of type Unknown(...)");
        }

        let is_default = variant.attrs.iter().any(|a| a.path().is_ident("default"));

        let discr = variant
            .discriminant
            .as_ref()
            .and_then(|(_, expr)| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit),
                    ..
                }) = expr
                {
                    Some(lit.base10_parse::<u8>().unwrap())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "The {} variant must have an explicit discriminant",
                    var_name
                )
            });

        if is_default {
            if default_variant.is_some() {
                panic!("Multiple variants marked with #[default]");
            }
            default_variant = Some((var_name, discr));
        }

        match_arms.push(quote! { #discr => #enum_name::#var_name, });
        as_u8_arms.push(quote! { #enum_name::#var_name => #discr, });
    }

    if !has_unknown {
        panic!("The enum must contain a variant Unknown(...)");
    }

    let default_impl = if let Some((var_name, _discr)) = default_variant {
        quote! {
            impl Default for #enum_name {
                fn default() -> Self {
                    #enum_name::#var_name
                }
            }
        }
    } else {
        quote! {
            impl Default for #enum_name {
                fn default() -> Self {
                    #enum_name::Unknown(0)
                }
            }
        }
    };

    let expanded = quote! {
        impl From<u8> for #enum_name {
            fn from(value: u8) -> Self {
                match value {
                    #(#match_arms)*
                    _ => #enum_name::Unknown(value),
                }
            }
        }

        impl #enum_name {
            pub fn as_u8(&self) -> u8 {
                match self {
                    #(#as_u8_arms)*
                    #enum_name::Unknown(u) => *u,
                }
            }
        }

        #default_impl

        impl serde::Serialize for #enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u8(self.as_u8())
            }
        }

        impl<'de> serde::Deserialize<'de> for #enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u8::deserialize(deserializer)?;
                Ok(Self::from(value))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(EnumFromString, attributes(str_value))]
pub fn derive_enum_from_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("EnumFromString can only be used on enums"),
    };

    let mut as_str_arms = Vec::new();
    let mut from_str_raw_arms = Vec::new();
    let mut from_str_result_arms = Vec::new();

    for variant in variants {
        let var_ident = &variant.ident;
        let lit = variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("str_value"))
            .map(|attr| {
                attr.parse_args::<syn::LitStr>()
                    .expect("str_value must be a string literal")
                    .value()
            })
            .unwrap_or_else(|| var_ident.to_string().to_lowercase());

        as_str_arms.push(quote! {
            #enum_name::#var_ident => #lit,
        });

        from_str_raw_arms.push(quote! {
            #lit => #enum_name::#var_ident,
        });

        from_str_result_arms.push(quote! {
            #lit => Ok(#enum_name::#var_ident),
        });
    }

    let expanded = quote! {
        impl #enum_name {
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_arms)*
                }
            }

            pub fn from_str(s: &str) -> Self {
                match s {
                    #(#from_str_raw_arms)*
                    _ => #enum_name::Unknown,
                }
            }
        }

        impl std::str::FromStr for #enum_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#from_str_result_arms)*
                    _ => Err(()),
                }
            }
        }

        impl serde::Serialize for #enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for #enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(Self::from_str(&s))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Flags, attributes(flag_enum))]
pub fn derive_flags(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_flags_impl(input)
}

fn generate_flags_impl(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let mut output = quote! {};

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let flag_enum_attr = field.attrs.iter()
                    .find(|attr| attr.path().is_ident("flag_enum"));

                if let Some(attr) = flag_enum_attr {
                    let is_option = is_option_u64_type(field_type);
                    let is_u64 = is_u64_type(field_type);

                    if !is_option && !is_u64 {
                        return syn::Error::new_spanned(
                            field_type,
                            "Field with #[flag_enum] must be of type u64 or Option<u64>"
                        ).to_compile_error().into();
                    }

                    let flags_content = attr.parse_args::<syn::LitStr>()
                        .expect("flag_enum attribute must contain a string with flag definitions")
                        .value();

                    let enum_name = format_ident!("{}{}", 
                        struct_name.to_string(),
                        field_name.to_string().to_case(Case::Pascal));

                    let flag_enum = generate_simple_flag_enum(&enum_name, &flags_content);
                    let conversion_methods = generate_flag_conversion_methods(
                        struct_name, field_name, &enum_name, is_option
                    );

                    output = quote! {
                        #output
                        #flag_enum
                        #conversion_methods
                    };
                }
            }
        }
    }

    output.into()
}

fn generate_simple_flag_enum(enum_name: &syn::Ident, flags_content: &str) -> proc_macro2::TokenStream {
    let flag_data: Vec<_> = flags_content.split(',')
        .map(|s| {
            let parts: Vec<_> = s.trim().split('=').collect();
            let name = format_ident!("{}", parts[0].trim());
            let value: u8 = parts[1].trim().parse().expect("Invalid flag value");
            (name, value)
        })
        .collect();

    let flag_variants = flag_data.iter().map(|(name, value)| {
        quote! { #name = #value }
    });

    let extract_flag_checks = flag_data.iter().map(|(name, value)| {
        quote! {
            if value & (1u64 << #value) != 0 {
                flags.push(#enum_name::#name);
            }
        }
    });

    let from_bit_arms = flag_data.iter().map(|(name, value)| {
        quote! { #value => Some(#enum_name::#name) }
    });

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum #enum_name {
            #(#flag_variants,)*
        }
        
        impl #enum_name {
            pub fn bit_value(&self) -> u64 {
                1u64 << (*self as u8)
            }
            
            pub fn from_bit_position(bit: u8) -> Option<Self> {
                match bit {
                    #(#from_bit_arms,)*
                    _ => None,
                }
            }
            
            pub fn extract_flags(value: u64) -> Vec<Self> {
                let mut flags = Vec::new();
                #(#extract_flag_checks)*
                flags
            }
            
            pub fn combine_flags(flags: &[Self]) -> u64 {
                flags.iter().fold(0u64, |acc, flag| acc | flag.bit_value())
            }
        }
    }
}

fn generate_flag_conversion_methods(
    struct_name: &syn::Ident,
    field_name: &syn::Ident,
    enum_name: &syn::Ident,
    is_option: bool,
) -> proc_macro2::TokenStream {
    let getter_name = format_ident!("get_{}", field_name);
    let setter_name = format_ident!("set_{}", field_name);
    let has_flag_name = format_ident!("has_{}", field_name);
    let add_flag_name = format_ident!("add_{}", field_name);
    let remove_flag_name = format_ident!("remove_{}", field_name);

    if is_option {
        quote! {
            impl #struct_name {
                pub fn #getter_name(&self) -> Vec<#enum_name> {
                    self.#field_name
                        .map(|value| #enum_name::extract_flags(value))
                        .unwrap_or_default()
                }
                
                pub fn #setter_name(&mut self, flags: Vec<#enum_name>) {
                    if flags.is_empty() {
                        self.#field_name = None;
                    } else {
                        self.#field_name = Some(#enum_name::combine_flags(&flags));
                    }
                }
                
                pub fn #has_flag_name(&self, flag: #enum_name) -> bool {
                    self.#field_name
                        .map(|value| value & flag.bit_value() != 0)
                        .unwrap_or(false)
                }
                
                pub fn #add_flag_name(&mut self, flag: #enum_name) {
                    let current = self.#field_name.unwrap_or(0);
                    self.#field_name = Some(current | flag.bit_value());
                }
                
                pub fn #remove_flag_name(&mut self, flag: #enum_name) {
                    if let Some(current) = self.#field_name {
                        let new_value = current & !flag.bit_value();
                        self.#field_name = if new_value == 0 { None } else { Some(new_value) };
                    }
                }
            }
        }
    } else {
        quote! {
            impl #struct_name {
                pub fn #getter_name(&self) -> Vec<#enum_name> {
                    #enum_name::extract_flags(self.#field_name)
                }
                
                pub fn #setter_name(&mut self, flags: Vec<#enum_name>) {
                    self.#field_name = #enum_name::combine_flags(&flags);
                }
                
                pub fn #has_flag_name(&self, flag: #enum_name) -> bool {
                    self.#field_name & flag.bit_value() != 0
                }
                
                pub fn #add_flag_name(&mut self, flag: #enum_name) {
                    self.#field_name |= flag.bit_value();
                }
                
                pub fn #remove_flag_name(&mut self, flag: #enum_name) {
                    self.#field_name &= !flag.bit_value();
                }
            }
        }
    }
}
