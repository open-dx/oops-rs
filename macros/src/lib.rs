use quote::quote;

//--
use syn::DeriveInput;

use syn::Data;
use syn::DataEnum;

use syn::Fields;
use syn::FieldsUnnamed;

use syn::Attribute;
use syn::Ident;
use syn::Meta;

//---
/// Derives an Error-type for an enum of error variants.
/// 
/// ### Example:
/// ```rust
/// #[derive(Error)]
/// enum SomeError {
///     /// This comment will be used as this
///     /// variant's interpolation values: {0:?}
///     FailedSomehow(String),
/// }
/// ```
#[proc_macro_derive(Error, attributes(code, msg, tags))]
pub fn derive_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    
    let error_impl = match ast.data {
        // TODO
        Data::Enum(ref enum_node) => derive_error_from_enum(ast.ident, &enum_node),
        
        // TODO
        _ => panic!("Error macro can only be used on enums"),
    };
    
    // Generate the impl block for the `ErrorCode` trait.
    proc_macro2::TokenStream::from(error_impl).into()
}

/// Generates a token stream which implements `Error`, as well as
/// the debug and display traits.
fn derive_error_from_enum(ident: Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    let display_match_arms = data.variants.iter().map(|variant| {
        create_display_arm(ident.clone(), variant)
    });
    
    let debug_match_arms = data.variants.iter().map(|variant| {
        // TODO: Write `create_debug_arm(..)`, too.
        create_display_arm(ident.clone(), variant)
    });
    
    quote! {
        impl core::error::Error for #ident {}
        
        impl core::fmt::Display for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", match self {
                    // Expand the match arms as strings.
                    #(#display_match_arms),*
                })
            }
        }
        
        impl core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", match self {
                    // Expand the match arms as strings.
                    #(#debug_match_arms),*
                })
            }
        }
    }
}

//---
/// Generates a token stream for a match arm for message display.
fn create_display_arm(root_ident: syn::Ident, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let msg_text_fallback = variant.ident.to_string();
    
    // Attempt to get the msg attribute's value, the doc attribute value, or
    // fallback to the variant identy name.
    let variant_msg = find_attr_value(&variant.attrs, "msg")
        .or_else(|| find_attr_value(&variant.attrs, "doc"))
        .unwrap_or(msg_text_fallback);
    
    match &variant.fields {
        // For Units, we just need 
        Fields::Unit => create_unit_match_arm(root_ident, &variant.ident, &variant_msg),
        Fields::Unnamed(fields) => create_tuple_match_arm(root_ident, &variant.ident, &variant_msg, fields),
        _ => panic!("Error macro does not support named or unit struct variants"),
    }
}

/// Generates a token stream for a match arm with no embedded values.
fn create_unit_match_arm(root_ident: syn::Ident, variant_ident: &syn::Ident, variant_msg: &str) -> proc_macro2::TokenStream {
    quote! {
        #root_ident::#variant_ident => format!(#variant_msg)
    }
}

/// Creates a token stream for a match arm for a variant with one or more values.
fn create_tuple_match_arm(root_ident: syn::Ident, variant_ident: &syn::Ident, variant_msg: &str, fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let field_names = (0..fields.unnamed.len())
        .map(|i| syn::Ident::new(&format!("x{}", i), proc_macro2::Span::call_site()));
    
    let field_values = field_names.clone()
        .map(|name| quote! { #name });

    quote! {
        #root_ident::#variant_ident(#(#field_names),*) => format!(#variant_msg, #(#field_values),*)
    }
}

/// Find an appropriately configured message value in a set of attributes.
/// Hint: The "doc" attribute is derived for 
fn find_attr_value(attrs: &[Attribute], name: &str) -> Option<String> {
    for attr in attrs {
        let attr_meta = attr.parse_meta();
        if let Err(error) = attr_meta {
            eprintln!("Couldn't parse meta attribute `{}`: {}", name, error);
            continue;
        }
        
        let attr_meta = attr_meta.unwrap();
        match attr_meta {
            Meta::Path(_) => {
                // TODO!
                continue;
            },
            Meta::List(list) => {
                if list.path.is_ident(name) {
                    if let Some(syn::NestedMeta::Lit(lit)) = list.nested.first() {
                        if let syn::Lit::Str(lit) = lit {
                            return Some(lit.value());
                        }
                    }
                }
            },
            Meta::NameValue(name_value) => {
                if name_value.path.is_ident(name) {
                    if let syn::Lit::Str(lit) = name_value.lit {
                        return Some(lit.value());
                    }
                }
            },
        }
    }
    
    None
}

//---
/// TODO
#[proc_macro_derive(Diagnostic, attributes(metric))]
pub fn derive_diagnostics(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new() // TODO
}
