use crate::{
    derive::{DataStructure, DataVariant},
    TokenStream2,
};

use syn::{
    punctuated::Punctuated,
    self, Attribute, Data, DeriveInput, Field as SynField, Fields as SynFields, Generics, Ident,
    Type, Visibility,
};

use quote::quote;


pub(crate) fn derive_impl(di: DeriveInput) -> syn::Result<TokenStream2> {
    let ds = DataStructure::new(&di);
    let name = ds.name;

    if ds.data_variant != DataVariant::Struct {
        return Err(syn::Error::new(name.span(), "Only structs are supported right now"));
    }

    let ty_params = ds.generics.type_params().map(|x| &x.ident);
    let field_names = ds.variants[0].fields.iter().map(|f| &f.ident);
    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();
    let preds = Punctuated::new(); 
    let preds = where_clause.map_or(&preds, |x| &x.predicates).into_iter();
    
    Ok(quote! {
        const _: () = {
            use ::core_extensions as __ce_bCj7dq3Pud;

            impl #impl_generics __ce_bCj7dq3Pud::ConstDefault for #name #ty_generics
            where
                #( #preds, )*
                #( #ty_params: __ce_bCj7dq3Pud::ConstDefault, )*
            {
                const DEFAULT: Self = Self {
                    #( #field_names: __ce_bCj7dq3Pud::ConstDefault::DEFAULT, )*
                };
            }
        };
    })
}


















