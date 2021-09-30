use crate::{
    derive::{DataStructure, DataVariant},
    TokenStream2,
};

use syn::{
    punctuated::Punctuated,
    DeriveInput,
};

use quote::quote;

#[cfg(test)]
use alloc::string::{String, ToString};


mod tn_attribute_parsing;

#[cfg(test)]
mod tn_tests;

use self::tn_attribute_parsing::WrappedFieldTranparency;


pub(crate) fn derive_impl(di: DeriveInput) -> syn::Result<TokenStream2> {
    let ds = &DataStructure::new(&di);
    let name = ds.name;

    if ds.data_variant != DataVariant::Struct {
        return Err(syn::Error::new(name.span(), "Only structs are supported"));
    }

    let config = tn_attribute_parsing::parse_attributes(ds)?;
    let extra_predicates = config.shared.extra_predicates.into_iter();
    let crate_path = config.shared.crate_path;
    let field_cfg = config.field;
    let field_ty = field_cfg.field.ty;

    let mut delegated_bound = TokenStream2::new();

    let inside_impl = match field_cfg.transparency {
        WrappedFieldTranparency::Direct =>
            quote!(
                type Inner = #field_ty;

                __ce_bCj7dq3Pud::impl_transparent_newtype!{Self}
            ),
        WrappedFieldTranparency::Delegated => {
            delegated_bound = quote!(#field_ty: __ce_bCj7dq3Pud::TransparentNewtype,);
            quote!(
                __ce_bCj7dq3Pud::delegate_transparent_newtype_impl!{Self, #field_ty}
            )
        }
    };

    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();
    let preds = Punctuated::new(); 
    let preds = where_clause.map_or(&preds, |x| &x.predicates).into_iter();
    
    let ret = quote! {
        const _: () = {
            use #crate_path as __ce_bCj7dq3Pud;

            unsafe impl #impl_generics __ce_bCj7dq3Pud::TransparentNewtype for #name #ty_generics
            where
                #( #preds, )*
                #( #extra_predicates, )*
                #delegated_bound
            {
                #inside_impl
            }
        };
    };

    if config.shared.debug_print {
        core::panic!("{}", ret);
    }

    Ok(ret)
}


#[cfg(test)]
pub(crate) fn derive_for_tests(input: &str) -> Result<String, String> {
    syn::parse_str(input)
        .and_then(crate::derive::transparent_newtype_derive::derive_impl)
        .map_err(syn::Error::into_compile_error)
        .map(|x| x.to_string())
        .map_err(|x| x.to_string())
}
