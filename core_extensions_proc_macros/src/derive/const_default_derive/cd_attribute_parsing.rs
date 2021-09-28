use super::{Bounds, DefaultVal, TypeBounds};

use crate::{
    derive::{DataStructure, DataVariant, Field, ParseBufferExt, Struct, SynResultExt},
    TokenStream2,
    mmatches,
};

use proc_macro2::Span;

use syn::{
    parse::ParseBuffer,
    punctuated::Punctuated,
    Attribute, Ident, Token,
};

use quote::quote;

use alloc::vec::Vec;
use alloc::vec;



#[derive(Copy, Clone)]
enum ParseCtx<'a> {
    Container,
    Variant(usize, &'a Struct<'a>),
    Field(&'a Field<'a>),
}

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
    extra_predicates: Punctuated<syn::WherePredicate, Token!(,)>,    
    debug_print: bool,
    crate_path: syn::Path,
}

pub(super) struct Configuration<'a> {
    pub(super) type_param_bounds: Vec<TokenStream2>,
    pub(super) field_bounds: Vec<TokenStream2>,
    pub(super) field_values: TokenStream2,
    pub(super) variant: Option<&'a Ident>,
    pub(super) extra_predicates: Punctuated<syn::WherePredicate, Token!(,)>,    
    pub(super) debug_print: bool,
    pub(super) crate_path: syn::Path,
}

impl<'a> ParsedAttributes<'a> {
    fn finish(self, ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
        let Self {
            mut type_param_bounds,
            variant,
            field_bound_attr,
            extra_predicates,
            debug_print,
            crate_path,
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
            extra_predicates,
            debug_print,
            crate_path,
        })
    }

    fn init_fields(&mut self, index: usize, struct_: &'a Struct<'a>) {
        self.variant = Some(VariantAttributes{
            index,
            field_bounds: vec![None; struct_.fields.len()],
            field_values: vec![DefaultVal::ConstDefault; struct_.fields.len()],
        });
    }
}


pub(super) fn parse_attributes<'a>(ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
    let mut res = syn::Result::Ok(());
    let mut this = ParsedAttributes{
        type_param_bounds: ds.generics
            .type_params()
            .map(|tp| (&tp.ident, Some(Bounds::ConstDefault)))
            .collect(),
        field_bound_attr: None,
        variant: None,
        extra_predicates: Punctuated::new(),
        debug_print: false,
        crate_path: syn::parse_quote!(::core_extensions),
    };

    if ds.data_variant == DataVariant::Struct {
        this.init_fields(0, &ds.variants[0]);
    }

    for attr in ds.attrs {
        res.combine_err(parse_attribute(&mut this, ds, ParseCtx::Container, attr));
    }

    if ds.data_variant == DataVariant::Enum {
        for (i, v) in ds.variants.iter().enumerate() {
            let ctx = ParseCtx::Variant(i, v);
            for attr in v.attrs {
                res.combine_err(parse_attribute(&mut this, ds, ctx, attr));
            }
        }
    }

    for v in &ds.variants {
        for f in &v.fields {
            for attr in f.attrs {
                res.combine_err(parse_attribute(&mut this, ds, ParseCtx::Field(f), attr));
            }
        }
    }

    res?;

    this.finish(ds)
}


mod keyword {
    syn::custom_keyword!(bound);
    syn::custom_keyword!(default);
    syn::custom_keyword!(field_bound);
    syn::custom_keyword!(no_bounds);
    syn::custom_keyword!(debug_print);
}

fn parse_attribute<'a>(
    this: &mut ParsedAttributes<'a>,
    ds: &'a DataStructure<'a>,
    ctx: ParseCtx<'a>,
    attribute: &Attribute,
) -> syn::Result<()> {
    if attribute.path.is_ident("cdef") {
        attribute.parse_args_with(move|input: &'_ ParseBuffer<'_>| {
            parse_attribute_inner(this, ds, ctx, input)
        })
    } else {
        Ok(())
    }
}

fn parse_attribute_inner<'a>(
    this: &mut ParsedAttributes<'a>,
    _ds: &'a DataStructure<'a>,
    ctx: ParseCtx<'a>,
    input: &'_ ParseBuffer<'_>,
) -> syn::Result<()> {
    if let Some(kw) = input.peek_parse(keyword::bound)? {
        check_is_container(&ctx, kw.span)?;

        let content;
        syn::parenthesized!(content in input);
        let ident = content.parse::<syn::Ident>()?;
        content.parse::<Token!(:)>()?;
        let bounds = TypeBounds::parse_terminated(&content)?;

        if let Some((_, bs)) = 
            this.type_param_bounds
                .iter_mut()
                .find(|(tp_name, _)| **tp_name == ident)
        {
            *bs = Some(Bounds::Custom(bounds));
        } else {
            return Err(syn::Error::new(ident.span(), "Expected a type parameter"))
        }
    } else if let Some(kw) = input.peek_parse(keyword::no_bounds)? {
        check_is_container(&ctx, kw.span)?;
        
        for (_, bs) in &mut this.type_param_bounds {
            *bs = None;
        }
    } else if let Some(kw) = input.peek_parse(keyword::field_bound)? {
        match ctx {
            ParseCtx::Container => {
                this.field_bound_attr = Some(FieldBoundAttr::Container);
            }
            ParseCtx::Variant(i, _) => {
                this.field_bound_attr = Some(FieldBoundAttr::Variant(i, kw.span));                
            },
            ParseCtx::Field(f) => {
                let va = check_valid_field_attr(&mut this.variant, f, kw.span)?;
                va.field_bounds[f.index.pos] = Some(Bounds::ConstDefault);
            }
        }
    } else if let Some(kw) = input.peek_parse(keyword::default)? {
        match ctx {
            ParseCtx::Container => check_is_variant_or_field(&ctx, kw.span)?,
            ParseCtx::Variant(index, v) => {
                if this.variant.is_some() {
                    return Err(syn::Error::new(kw.span, "Cannot set the default variant twice"))
                }

                this.init_fields(index, v);
            }
            ParseCtx::Field(f) => {
                let va = check_valid_field_attr(&mut this.variant, f, kw.span)?;

                input.parse::<Token!(=)>()?;
                let expr = input.parse::<TokenStream2>()?;

                va.field_values[f.index.pos] = DefaultVal::Custom{
                    expr,
                    paren_span: input.span(),
                };
            }
        }
    } else if let Some(_) = input.peek_parse(Token!(where))? {
        if !input.is_empty() {
            loop{
                this.extra_predicates.push(input.parse::<syn::WherePredicate>()?);
                if input.is_empty() { break; }
                input.parse::<Token!(,)>()?;
            }
        }
    } else if let Some(_) = input.peek_parse(keyword::debug_print)? {
        this.debug_print = true;
    } else if let Some(_) = input.peek_parse(Token!(crate))? {
        input.parse::<Token!(=)>()?;
        this.crate_path = input.parse::<syn::Path>()?;
    } else {
        let span = input.parse::<syn::Ident>()?.span();
        return Err(syn::Error::new(span, "Invalid attribute"));
    }
    Ok(())
}





fn check_is_container(ctx: &ParseCtx<'_>, span: Span) -> syn::Result<()> {
    if mmatches!(ctx, ParseCtx::Container) {
        Ok(())
    } else {
        Err(syn::Error::new(span, "Can only use this attribute above the type definition"))
    }
}


fn check_is_variant_or_field(ctx: &ParseCtx<'_>, span: Span) -> syn::Result<()> {
    if mmatches!(ctx, ParseCtx::Container) {
        Err(syn::Error::new(span, "Can only use this attribute on variants of fields"))
    } else {
        Ok(())
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


