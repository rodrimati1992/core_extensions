use crate::derive::{
    attr_parsing::{self, AttrParsing, SharedConfig, ParseCtx},
    utils::Empty,
    DataStructure, Field, ParseBufferExt,
};

use proc_macro2::Span;

use syn::{
    parse::ParseBuffer,
    Attribute,
};


pub(super) struct WrappedField<'a> {
    pub(super) field: &'a Field<'a>,
    pub(super) transparency: WrappedFieldTranparency,
}

pub(super) enum WrappedFieldTranparency {
    Direct,
    Delegated,
}

struct ParsedAttributes<'a> {
    field: Option<WrappedField<'a>>,
    has_transparent_repr: bool,
    shared: SharedConfig,
}

pub(super) struct Configuration<'a> {
    pub(super) field: WrappedField<'a>,
    pub(super) shared: SharedConfig,
}


pub(super) fn parse_attributes<'a>(ds: &'a DataStructure<'a>) -> syn::Result<Configuration<'a>> {
    let mut this = ParsedAttributes{
        field: None,
        has_transparent_repr: false,
        shared: SharedConfig::new(),
    };

    if let [field] = &ds.variants[0].fields[..] {
        this.field = Some(WrappedField{
            field,
            transparency: WrappedFieldTranparency::Direct,
        })
    }

    this.parse_item_attributes(ds)
}

mod keyword {
    syn::custom_keyword!(delegate);
    syn::custom_keyword!(transparent);
}

impl<'a> AttrParsing<'a> for ParsedAttributes<'a> {
    type Config = Configuration<'a>;
    const HELPER_ATTR: &'static str = "twrap";

    fn shared_config_mut(&mut self) -> &mut SharedConfig {
        &mut self.shared
    }

    fn parse_helper_attribute(
        &mut self,
        _ds: &'a DataStructure<'a>,
        ctx: ParseCtx<'a>,
        input: &'_ ParseBuffer<'_>,
    ) -> syn::Result<()> {
        let field = attr_parsing::check_is_field(ctx, &Empty(input.span()))?;

        let mut assign_field = |transparency| {
            self.field = Some(WrappedField{field, transparency});
        };

        if let Some(_) = input.peek_parse(keyword::delegate)? {
            assign_field(WrappedFieldTranparency::Delegated);
        } else if input.is_empty() {
            assign_field(WrappedFieldTranparency::Direct);
        } else {
            return Err(input.error("expected either `#[twrap(delegate)]` or `#[twrap]`"));
        }
        Ok(())
    }

    fn parse_other_container_attr(
        &mut self,
        _ds: &'a DataStructure<'a>,
        attribute: &Attribute,
    ) -> syn::Result<()> {
        if attribute.path.is_ident("repr") {
            attribute.parse_args_with(move|input: &'_ ParseBuffer<'_>| {
                input.parse::<keyword::transparent>()?;

                self.has_transparent_repr = true;
                Ok(())
            })
        } else {
            Ok(())
        }
    }

    fn finish(self, _de: &'a DataStructure<'a>) -> syn::Result<Self::Config> {
        let field = self.field.ok_or_else(||{
            syn::Error::new(
                Span::call_site(),
                "Expected a `#[twrap]` attribute on exactly one field",
            )
        })?;

        if !self.has_transparent_repr {
            let msg = "This type must have a `#[repr(transaprent)]` representation attribute.";
            return Err(syn::Error::new(Span::call_site(), msg));
        }

        Ok(Configuration{
            field,
            shared: self.shared,
        })
    }
}

