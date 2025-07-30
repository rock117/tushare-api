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

/// Derive macro for automatically implementing TryFrom<TushareResponse> for wrapper struct
/// 
/// This macro generates a wrapper type that contains the converted data items along with
/// pagination information (has_more and count fields from TushareData).
/// 
/// # Generated Structure
/// 
/// For a struct named `Stock`, this macro generates `StockList` with:
/// - `items: Vec<Stock>` - The converted data items
/// - `has_more: bool` - Whether more pages are available
/// - `count: i64` - Total number of records
/// 
/// # Example
/// 
/// ```rust
/// use tushare_derive::{FromTushareData, TushareResponseList};
/// 
/// #[derive(FromTushareData, TushareResponseList)]
/// struct Stock {
///     ts_code: String,
///     name: String,
/// }
/// 
/// // This generates:
/// // pub struct StockList {
/// //     pub items: Vec<Stock>,
/// //     pub has_more: bool,
/// //     pub count: i64,
/// // }
/// // impl TryFrom<TushareResponse> for StockList { ... }
/// 
/// // Usage:
/// let stock_list: StockList = response.try_into()?;
/// println!("Got {} stocks, has_more: {}, total: {}", 
///          stock_list.len(), stock_list.has_more(), stock_list.count());
/// 
/// // Access items directly via Deref
/// for stock in &stock_list {
///     println!("{}: {}", stock.ts_code, stock.name);
/// }
/// ```
#[proc_macro_derive(TushareResponseList)]
pub fn derive_tushare_response_list(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let list_name = syn::Ident::new(&format!("{}List", name), name.span());
    
    let expanded = quote! {
        /// Auto-generated wrapper type for converting TushareResponse to Vec<#name>
        /// 
        /// This struct contains:
        /// - `items`: The list of converted data items
        /// - `has_more`: Whether there are more pages available
        /// - `count`: Total number of records available
        #[derive(Debug, Clone)]
        pub struct #list_name {
            /// The list of converted data items
            pub items: Vec<#name>,
            /// Whether there are more pages available
            pub has_more: bool,
            /// Total number of records available
            pub count: i64,
        }
        
        impl #list_name {
            /// Create a new instance with items and pagination info
            pub fn new(items: Vec<#name>, has_more: bool, count: i64) -> Self {
                Self { items, has_more, count }
            }
            
            /// Get the items as a slice
            pub fn items(&self) -> &[#name] {
                &self.items
            }
            
            /// Get mutable reference to items
            pub fn items_mut(&mut self) -> &mut Vec<#name> {
                &mut self.items
            }
            
            /// Check if there are more pages available
            pub fn has_more(&self) -> bool {
                self.has_more
            }
            
            /// Get the total count of records
            pub fn count(&self) -> i64 {
                self.count
            }
            
            /// Get the number of items in current page
            pub fn len(&self) -> usize {
                self.items.len()
            }
            
            /// Check if the current page is empty
            pub fn is_empty(&self) -> bool {
                self.items.is_empty()
            }
        }
        
        impl std::ops::Deref for #list_name {
            type Target = Vec<#name>;
            
            fn deref(&self) -> &Self::Target {
                &self.items
            }
        }
        
        impl std::ops::DerefMut for #list_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.items
            }
        }
        
        impl TryFrom<tushare_api::types::TushareResponse> for #list_name {
            type Error = tushare_api::error::TushareError;
            
            fn try_from(response: tushare_api::types::TushareResponse) -> Result<Self, Self::Error> {
                let items = tushare_api::utils::response_to_vec::<#name>(response.clone())?;
                Ok(#list_name {
                    items,
                    has_more: response.data.has_more,
                    count: response.data.count,
                })
            }
        }
        
        impl From<Vec<#name>> for #list_name {
            fn from(items: Vec<#name>) -> Self {
                let count = items.len() as i64;
                #list_name {
                    items,
                    has_more: false,
                    count,
                }
            }
        }
        
        impl IntoIterator for #list_name {
            type Item = #name;
            type IntoIter = std::vec::IntoIter<#name>;
            
            fn into_iter(self) -> Self::IntoIter {
                self.items.into_iter()
            }
        }
        
        impl<'a> IntoIterator for &'a #list_name {
            type Item = &'a #name;
            type IntoIter = std::slice::Iter<'a, #name>;
            
            fn into_iter(self) -> Self::IntoIter {
                self.items.iter()
            }
        }
        
        impl<'a> IntoIterator for &'a mut #list_name {
            type Item = &'a mut #name;
            type IntoIter = std::slice::IterMut<'a, #name>;
            
            fn into_iter(self) -> Self::IntoIter {
                self.items.iter_mut()
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
