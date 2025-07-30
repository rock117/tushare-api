use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

/// Derive macro for automatically implementing FromTushareData trait
/// 
/// This macro generates the implementation of FromTushareData trait for structs,
/// enabling automatic conversion from Tushare API response data to Rust structs.
/// 
/// # Attributes
/// 
/// - `#[tushare(field = "api_field_name")]` - Maps struct field to a different API field name
/// - `#[tushare(skip)]` - Skips this field during conversion (field must have Default implementation)
/// 
/// # Example
/// 
/// ```rust
/// use tushare_derive::FromTushareData;
/// 
/// #[derive(FromTushareData)]
/// struct Stock {
///     ts_code: String,
///     symbol: String,
///     name: String,
///     area: Option<String>,
///     #[tushare(field = "list_date")]
///     listing_date: Option<String>,
///     #[tushare(skip)]
///     calculated_field: f64,
/// }
/// ```
#[proc_macro_derive(FromTushareData, attributes(tushare))]
pub fn derive_from_tushare_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromTushareData can only be derived for structs with named fields"),
        },
        _ => panic!("FromTushareData can only be derived for structs"),
    };

    let field_assignments = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        
        // Check for tushare attributes
        let mut api_field_name = field_name.as_ref().unwrap().to_string();
        let mut skip_field = false;
        
        for attr in &field.attrs {
            if attr.path().is_ident("tushare") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    for nested in meta_list.tokens.clone().into_iter() {
                        let nested_str = nested.to_string();
                        if nested_str.starts_with("field") {
                            // Extract field name from field = "name"
                            if let Some(start) = nested_str.find('"') {
                                if let Some(end) = nested_str.rfind('"') {
                                    if start < end {
                                        api_field_name = nested_str[start + 1..end].to_string();
                                    }
                                }
                            }
                        } else if nested_str == "skip" {
                            skip_field = true;
                        }
                    }
                }
            }
        }
        
        if skip_field {
            quote! {
                #field_name: Default::default(),
            }
        } else {
            // Generate field assignment based on type
            if is_option_type(field_type) {
                let inner_type = extract_option_inner_type(field_type);
                if is_string_type(&inner_type) {
                    quote! {
                        #field_name: tushare_api::utils::get_optional_string_field(fields, values, #api_field_name)?,
                    }
                } else if is_float_type(&inner_type) {
                    quote! {
                        #field_name: tushare_api::utils::get_optional_float_field(fields, values, #api_field_name)?,
                    }
                } else if is_int_type(&inner_type) {
                    quote! {
                        #field_name: tushare_api::utils::get_optional_int_field(fields, values, #api_field_name)?,
                    }
                } else if is_bool_type(&inner_type) {
                    quote! {
                        #field_name: tushare_api::utils::get_optional_bool_field(fields, values, #api_field_name)?,
                    }
                } else {
                    quote! {
                        #field_name: tushare_api::utils::get_optional_string_field(fields, values, #api_field_name)?,
                    }
                }
            } else if is_string_type(field_type) {
                quote! {
                    #field_name: tushare_api::utils::get_string_field(fields, values, #api_field_name)?,
                }
            } else if is_float_type(field_type) {
                quote! {
                    #field_name: tushare_api::utils::get_float_field(fields, values, #api_field_name)?,
                }
            } else if is_int_type(field_type) {
                quote! {
                    #field_name: tushare_api::utils::get_int_field(fields, values, #api_field_name)?,
                }
            } else if is_bool_type(field_type) {
                quote! {
                    #field_name: tushare_api::utils::get_bool_field(fields, values, #api_field_name)?,
                }
            } else {
                // Default to string for unknown types
                quote! {
                    #field_name: tushare_api::utils::get_string_field(fields, values, #api_field_name)?,
                }
            }
        }
    });

    let expanded = quote! {
        impl tushare_api::traits::FromTushareData for #name {
            fn from_row(
                fields: &[String],
                values: &[serde_json::Value],
            ) -> Result<Self, tushare_api::error::TushareError> {
                Ok(Self {
                    #(#field_assignments)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}



// Helper functions for type checking
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn extract_option_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty.clone();
                    }
                }
            }
        }
    }
    // Fallback to String if we can't extract the inner type
    syn::parse_str("String").unwrap()
}

fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "String" || segment.ident == "str";
        }
    }
    false
}

fn is_float_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = &segment.ident;
            return ident == "f32" || ident == "f64";
        }
    }
    false
}

fn is_int_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = &segment.ident;
            return ident == "i8" || ident == "i16" || ident == "i32" || ident == "i64" 
                || ident == "u8" || ident == "u16" || ident == "u32" || ident == "u64"
                || ident == "isize" || ident == "usize";
        }
    }
    false
}

fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "bool";
        }
    }
    false
}
