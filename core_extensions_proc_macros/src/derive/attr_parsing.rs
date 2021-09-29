use crate::{
    derive::{DataStructure, DataVariant, Field, ParseBufferExt, Struct, SynResultExt},
    mmatches,
};

use syn::{
    parse::ParseBuffer,
    punctuated::Punctuated,
    spanned,
    Attribute, Token,
};


pub(crate) trait AttrParsing<'a>: Sized {
    type Config;
    const HELPER_ATTR: &'static str;

    fn parse_item_attributes(mut self, ds: &'a DataStructure<'a>) -> syn::Result<Self::Config> {
        let mut res = syn::Result::Ok(());

        for attr in ds.attrs {
            res.combine_err(self.parse_attribute(ds, ParseCtx::Container, attr));
        }

        if ds.data_variant == DataVariant::Enum {
            for (i, v) in ds.variants.iter().enumerate() {
                let ctx = ParseCtx::Variant(i, v);
                for attr in v.attrs {
                    res.combine_err(self.parse_attribute(ds, ctx, attr));
                }
            }
        }

        for v in &ds.variants {
            for f in &v.fields {
                for attr in f.attrs {
                    res.combine_err(self.parse_attribute(ds, ParseCtx::Field(f), attr));
                }
            }
        }

        res?;

        self.finish(ds)
    }

    fn parse_attribute(
        &mut self,
        ds: &'a DataStructure<'a>,
        ctx: ParseCtx<'a>,
        attribute: &Attribute,
    ) -> syn::Result<()> {
        if attribute.path.is_ident(Self::HELPER_ATTR) {
            attribute.parse_args_with(move|input: &'_ ParseBuffer<'_>| {
                if let Some(_) = input.peek_parse(Token!(where))? {
                    check_is_container(&ctx, &attribute.path)?;

                    if !input.is_empty() {
                        let this = self.shared_config_mut();
                        loop{
                            this.extra_predicates.push(input.parse::<syn::WherePredicate>()?);
                            if input.is_empty() { break; }
                            input.parse::<Token!(,)>()?;
                            if input.is_empty() { break; }
                        }
                    }
                } else if let Some(_) = input.peek_parse(keyword::debug_print)? {
                    check_is_container(&ctx, &attribute.path)?;

                    self.shared_config_mut().debug_print = true;
                } else if let Some(_) = input.peek_parse(Token!(crate))? {
                    check_is_container(&ctx, &attribute.path)?;

                    input.parse::<Token!(=)>()?;
                    self.shared_config_mut().crate_path = input.parse::<syn::Path>()?;
                } else {
                    self.parse_helper_attribute(ds, ctx, input)?;
                }
                Ok(())
            })
        } else {
            self.parse_other_top_level_attr(ds, ctx, attribute)
        }
    }

    fn parse_other_top_level_attr(
        &mut self,
        _ds: &'a DataStructure<'a>,
        _ctx: ParseCtx<'a>,
        _attribute: &Attribute,
    ) -> syn::Result<()> {
        Ok(())
    }

    fn parse_helper_attribute(
        &mut self,
        _ds: &'a DataStructure<'a>,
        ctx: ParseCtx<'a>,
        input: &'_ ParseBuffer<'_>,
    ) -> syn::Result<()>;

    fn finish(self, ds: &'a DataStructure<'a>) -> syn::Result<Self::Config>;

    fn shared_config_mut(&mut self) -> &mut SharedConfig;
}

pub(crate) struct SharedConfig {
    pub(crate) extra_predicates: Punctuated<syn::WherePredicate, Token!(,)>,    
    pub(crate) debug_print: bool,
    pub(crate) crate_path: syn::Path,
}

#[derive(Copy, Clone)]
pub(crate) enum ParseCtx<'a> {
    Container,
    Variant(usize, &'a Struct<'a>),
    Field(&'a Field<'a>),
}

impl SharedConfig {
    pub fn new() -> Self {
        Self {
            extra_predicates: Punctuated::new(),
            debug_print: false,
            crate_path: syn::parse_quote!(::core_extensions),
        }
    }
}



mod keyword {
    syn::custom_keyword!(debug_print);
}


pub(crate) fn check_is_container(
    ctx: &ParseCtx<'_>,
    sp: &dyn spanned::Spanned,
) -> syn::Result<()> {
    if mmatches!(ctx, ParseCtx::Container) {
        Ok(())
    } else {
        Err(syn::Error::new(sp.span(), "Can only use this attribute above the type definition"))
    }
}


pub(crate) fn check_is_variant_or_field(
    ctx: &ParseCtx<'_>, 
    sp: &dyn spanned::Spanned,
) -> syn::Result<()> {
    if mmatches!(ctx, ParseCtx::Container) {
        Err(syn::Error::new(sp.span(), "Can only use this attribute on variants of fields"))
    } else {
        Ok(())
    }
}

