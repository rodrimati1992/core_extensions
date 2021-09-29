use super::{Bounds, DefaultVal, TypeBounds};

use crate::{
    derive::{
        attr_parsing::{self, AttrParsing, SharedConfig, ParseCtx},
        DataStructure, DataVariant, Field, ParseBufferExt, Struct,
    },
    TokenStream2,
};

use proc_macro2::Span;

use syn::{
    parse::ParseBuffer,
    Ident, Token,
};

use quote::quote;

use alloc::vec::Vec;
use alloc::vec;

struct VariantAttributes {
    // which variant this is for
    index: usize,
    field_bounds: Vec<Option<Bounds>>,
    field_values: Vec<DefaultVal>,
}

enum FieldBoundAttr {
    Container,
    Variant(usize, Span),
}

struct ParsedAttributes<'a> {
    type_param_bounds: Vec<(&'a Ident, Option<Bounds>)>,
    variant: Option<VariantAttributes>,    
    field_bound_attr: Option<FieldBoundAttr>,
    shared: SharedConfig,
}

pub(super) struct Configuration<'a> {
    pub(super) type_param_bounds: Vec<TokenStream2>,
    pub(super) field_bounds: Vec<TokenStream2>,
    pub(super) field_values: TokenStream2,
    pub(super) variant: Option<&'a Ident>,
    pub(super) shared: SharedConfig,
}


pub(super) fn parse_attributes<'a>(ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
    let mut this = ParsedAttributes{
        type_param_bounds: ds.generics
            .type_params()
            .map(|tp| (&tp.ident, Some(Bounds::ConstDefault)))
            .collect(),
        field_bound_attr: None,
        variant: None,
        shared: SharedConfig::new(),
    };

    if ds.data_variant == DataVariant::Struct {
        this.init_fields(0, &ds.variants[0]);
    }    

    this.parse_item_attributes(ds)
}

mod keyword {
    syn::custom_keyword!(bound);
    syn::custom_keyword!(default);
    syn::custom_keyword!(field_bound);
    syn::custom_keyword!(no_bounds);
    syn::custom_keyword!(debug_print);
}

impl<'a> AttrParsing<'a> for ParsedAttributes<'a> {
    type Config = Configuration<'a>;
    const HELPER_ATTR: &'static str = "cdef";

    fn shared_config_mut(&mut self) -> &mut SharedConfig {
        &mut self.shared
    }

    fn parse_helper_attribute(
        &mut self,
        _ds: &'a DataStructure<'a>,
        ctx: ParseCtx<'a>,
        input: &'_ ParseBuffer<'_>,
    ) -> syn::Result<()> {
        if let Some(kw) = input.peek_parse(keyword::bound)? {
            attr_parsing::check_is_container(&ctx, &kw)?;

            let content;
            syn::parenthesized!(content in input);
            let ident = content.parse::<syn::Ident>()?;
            content.parse::<Token!(:)>()?;
            let bounds = TypeBounds::parse_terminated(&content)?;

            if let Some((_, bs)) = 
                self.type_param_bounds
                    .iter_mut()
                    .find(|(tp_name, _)| **tp_name == ident)
            {
                *bs = Some(Bounds::Custom(bounds));
            } else {
                return Err(syn::Error::new(ident.span(), "Expected a type parameter"))
            }
        } else if let Some(kw) = input.peek_parse(keyword::no_bounds)? {
            attr_parsing::check_is_container(&ctx, &kw)?;
            
            for (_, bs) in &mut self.type_param_bounds {
                *bs = None;
            }
        } else if let Some(kw) = input.peek_parse(keyword::field_bound)? {
            match ctx {
                ParseCtx::Container => {
                    self.field_bound_attr = Some(FieldBoundAttr::Container);
                }
                ParseCtx::Variant(i, _) => {
                    self.field_bound_attr = Some(FieldBoundAttr::Variant(i, kw.span));
                },
                ParseCtx::Field(f) => {
                    let va = check_valid_field_attr(&mut self.variant, f, kw.span)?;
                    va.field_bounds[f.index.pos] = Some(Bounds::ConstDefault);
                }
            }
        } else if let Some(kw) = input.peek_parse(keyword::default)? {
            match ctx {
                ParseCtx::Container => attr_parsing::check_is_variant_or_field(&ctx, &kw)?,
                ParseCtx::Variant(index, v) => {
                    if self.variant.is_some() {
                        return Err(syn::Error::new(kw.span, "Cannot set the default variant twice"))
                    }

                    self.init_fields(index, v);
                }
                ParseCtx::Field(f) => {
                    let va = check_valid_field_attr(&mut self.variant, f, kw.span)?;

                    input.parse::<Token!(=)>()?;
                    let expr = input.parse::<TokenStream2>()?;

                    va.field_values[f.index.pos] = DefaultVal::Custom{
                        expr,
                        paren_span: input.span(),
                    };
                }
            }
        } else {
            let span = input.parse::<syn::Ident>()?.span();
            return Err(syn::Error::new(span, "Invalid attribute"));
        }
        Ok(())
    }

    fn finish(self, ds: &'a DataStructure<'a>) -> syn::Result<Self::Config> {
        let Self {
            mut type_param_bounds,
            variant,
            field_bound_attr,
            shared,
        } = self;

        let mut variant = variant.ok_or_else(||syn::Error::new(
            Span::call_site(),
            "Expected a variant with a `#[cdef(default)]` attribute"
        ))?;
        
        match field_bound_attr {
            Some(FieldBoundAttr::Variant(n, span)) if variant.index != n  => {
                return Err(syn::Error::new(
                    span,
                    "Cannot use the `#[cdef(field_bound)]` attribute on a \
                     non-`#[cdef(default)] variant"
                ))
            }
            Some(FieldBoundAttr::Variant{..}) | Some(FieldBoundAttr::Container) => {
                for (_, bounds) in &mut type_param_bounds {
                    if let b @ Some(Bounds::ConstDefault) = bounds {
                        *b = None;
                    }
                }
                for bounds in &mut variant.field_bounds {
                    bounds.get_or_insert(Bounds::ConstDefault);
                }
            }
            None => {}
        }

        let type_param_bounds = type_param_bounds
            .into_iter()
            .filter_map(|(tp_name, tp_bounds)| tp_bounds.map(|x| (tp_name, x)))
            .map(|(tp_name, tp_bounds)| quote!(#tp_name: #tp_bounds))
            .collect::<Vec<TokenStream2>>();

        let struct_ = &ds.variants[variant.index];

        let field_bounds = variant.field_bounds
            .into_iter()
            .zip(&struct_.fields)
            .filter_map(|(bounds, f)| {
                let ty = f.ty;
                bounds.map(|b| quote!(#ty: #b))
            })
            .collect();

        let field_values = {
            let fi = struct_.fields.iter().map(|f|  &f.ident);
            let fv = variant.field_values.iter();
            quote!(#(#fi: #fv,)*)
        };

        let variant = Some(struct_.name).filter(|_| ds.data_variant == DataVariant::Enum);

        Ok(Configuration{
            type_param_bounds,
            field_bounds,
            field_values,
            variant,
            shared,
        })
    }
}

impl<'a> ParsedAttributes<'a> {
    fn init_fields(&mut self, index: usize, struct_: &'a Struct<'a>) {
        self.variant = Some(VariantAttributes {
            index,
            field_bounds: vec![None; struct_.fields.len()],
            field_values: vec![DefaultVal::ConstDefault; struct_.fields.len()],
        });
    }
}

fn check_valid_field_attr<'a>(
    this: &'a mut Option<VariantAttributes>,
    field: &Field<'_>,
    span: Span,
) -> syn::Result<&'a mut VariantAttributes> {
    let msg = match this {
        Some(va) if va.index == field.index.variant => return Ok(va),
        Some(_) => "Cannot use field attributes on non-`#[cdef(default)]` variant",
        None => "Cannot use field attributes on non-`#[cdef(default)]` variant",
    };

    Err(syn::Error::new(span, msg))
}


