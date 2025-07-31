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
/// - `#[tushare(date_format = "format_string")]` - Specifies custom date format for chrono date/time types
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
///     #[tushare(date_format = "%d/%m/%Y")]
///     custom_date: chrono::NaiveDate,
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
        let mut date_format: Option<String> = None;
        
        for attr in &field.attrs {
            if attr.path().is_ident("tushare") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    let tokens_str = meta_list.tokens.to_string();
                    
                    // Parse field = "value" pattern
                    if let Some(field_start) = tokens_str.find("field") {
                        let after_field = &tokens_str[field_start + 5..]; // Skip "field"
                        if let Some(eq_pos) = after_field.find('=') {
                            let after_eq = &after_field[eq_pos + 1..].trim();
                            if let Some(start_quote) = after_eq.find('"') {
                                let after_start_quote = &after_eq[start_quote + 1..];
                                if let Some(end_quote) = after_start_quote.find('"') {
                                    api_field_name = after_start_quote[..end_quote].to_string();
                                }
                            }
                        }
                    }
                    
                    // Check for skip attribute
                    if tokens_str.contains("skip") {
                        skip_field = true;
                    }
                    
                    // Parse date_format = "value" pattern
                    if let Some(format_start) = tokens_str.find("date_format") {
                        let after_format = &tokens_str[format_start + 11..]; // Skip "date_format"
                        if let Some(eq_pos) = after_format.find('=') {
                            let after_eq = &after_format[eq_pos + 1..].trim();
                            if let Some(start_quote) = after_eq.find('"') {
                                let after_start_quote = &after_eq[start_quote + 1..];
                                if let Some(end_quote) = after_start_quote.find('"') {
                                    date_format = Some(after_start_quote[..end_quote].to_string());
                                }
                            }
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
            // Generate field assignment using unified trait approach
            if is_option_type(field_type) {
                let inner_type = extract_option_inner_type(field_type);
                
                if let Some(format) = date_format {
                    // Use custom date format for optional types
                    quote! {
                        #field_name: {
                            let value = match tushare_api::utils::get_field_value(fields, values, #api_field_name) {
                                Ok(v) => v,
                                Err(_) => &serde_json::Value::Null,
                            };
                            tushare_api::traits::from_optional_tushare_value_with_date_format::<#inner_type>(value, #format)?
                        },
                    }
                } else {
                    // Use FromOptionalTushareValue trait for all Option<T> types
                    quote! {
                        #field_name: {
                            let value = match tushare_api::utils::get_field_value(fields, values, #api_field_name) {
                                Ok(v) => v,
                                Err(_) => &serde_json::Value::Null,
                            };
                            <#inner_type as tushare_api::traits::FromOptionalTushareValue>::from_optional_tushare_value(value)?
                        },
                    }
                }
            } else {
                if let Some(format) = date_format {
                    // Use custom date format for non-optional types
                    quote! {
                        #field_name: {
                            let value = tushare_api::utils::get_field_value(fields, values, #api_field_name)?;
                            tushare_api::traits::from_tushare_value_with_date_format::<#field_type>(value, #format)?
                        },
                    }
                } else {
                    // Use FromTushareValue trait for all non-optional types
                    quote! {
                        #field_name: {
                            let value = tushare_api::utils::get_field_value(fields, values, #api_field_name)?;
                            <#field_type as tushare_api::traits::FromTushareValue>::from_tushare_value(value)?
                        },
                    }
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

// Note: Type checking functions removed since we now use unified trait calls
// for all types through FromTushareValue and FromOptionalTushareValue
