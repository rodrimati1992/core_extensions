use crate::{
    derive::{DataStructure, DataVariant},
    TokenStream2,
};

use proc_macro2::Span;

use syn::{
    punctuated::Punctuated,
    DeriveInput,
};

use quote::{TokenStreamExt, ToTokens, quote};

#[cfg(test)]
use alloc::string::{String, ToString};


mod cd_attribute_parsing;

#[cfg(test)]
mod cd_tests;



pub(crate) fn derive_impl(di: DeriveInput) -> syn::Result<TokenStream2> {
    let ds = &DataStructure::new(&di);
    let name = ds.name;

    if ds.data_variant == DataVariant::Union {
        return Err(syn::Error::new(name.span(), "Only structs and enums are supported"));
    }

    let config = cd_attribute_parsing::parse_attributes(ds)?;
    let type_param_bounds = config.type_param_bounds.into_iter();
    let field_bounds = config.field_bounds.into_iter();
    let field_values = config.field_values;
    let extra_predicates = config.extra_predicates.into_iter();
    let crate_path = config.crate_path;
    let variant = config.variant.into_iter();

    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();
    let preds = Punctuated::new(); 
    let preds = where_clause.map_or(&preds, |x| &x.predicates).into_iter();
    
    let ret = quote! {
        const _: () = {
            use #crate_path as __ce_bCj7dq3Pud;

            impl #impl_generics __ce_bCj7dq3Pud::ConstDefault for #name #ty_generics
            where
                #( #preds, )*
                #( #type_param_bounds, )*
                #( #field_bounds, )*
                #( #extra_predicates, )*
            {
                const DEFAULT: Self = Self #(::#variant)* {
                    #field_values
                };
            }
        };
    };

    if config.debug_print {
        core::panic!("{}", ret);
    }

    Ok(ret)
}


#[cfg(test)]
pub(crate) fn derive_for_tests(input: &str) -> Result<String, String> {
    syn::parse_str(input)
        .and_then(crate::derive::const_default_derive::derive_impl)
        .map_err(syn::Error::into_compile_error)
        .map(|x| x.to_string())
        .map_err(|x| x.to_string())
}



/// Which bounds a thing has
#[derive(Clone)]
enum Bounds {
    ConstDefault,
    Custom(TypeBounds),
}

type TypeBounds = syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token!(+)>;

/// Which bounds a thing has
#[derive(Clone)]
enum DefaultVal {
    ConstDefault,
    Custom {
        expr: TokenStream2,
        paren_span: Span,
    },
}

impl ToTokens for Bounds {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match self {
            Bounds::ConstDefault => ts.append_all(quote!(__ce_bCj7dq3Pud::ConstDefault)),
            Bounds::Custom(bounds) => bounds.to_tokens(ts),
        }
    }
}


impl ToTokens for DefaultVal {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match self {
            DefaultVal::ConstDefault =>{
                ts.append_all(quote!(__ce_bCj7dq3Pud::ConstDefault::DEFAULT));
            }
            DefaultVal::Custom{expr, paren_span} => {
                ts.append_all(quote::quote_spanned!(*paren_span => (#expr)));
            }
        }
    }
}











