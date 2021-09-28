use syn::{
    self, Attribute, Data, DeriveInput, Field as SynField, Fields as SynFields, Generics, Ident,
    Type, Visibility,
};

use quote::{ToTokens, format_ident};

use proc_macro2::TokenStream;

use alloc::{
    format,
    vec::Vec,
};


//////////////////////////////////////////////////////////////////////////////


/// A type definition(enum,struct,union).
#[derive(Clone)]
pub struct DataStructure<'a> {
    pub vis: &'a Visibility,
    pub name: &'a Ident,
    pub generics: &'a Generics,
    pub attrs: &'a [Attribute],

    /// Whether this is a struct/union/enum.
    pub data_variant: DataVariant,

    /// The variants in the type definition.
    ///
    /// If it is a struct or a union this only has 1 element.
    pub variants: Vec<Struct<'a>>,
}

impl<'a> DataStructure<'a> {
    pub fn new(ast: &'a DeriveInput) -> Self {
        let name = &ast.ident;

        let data_variant: DataVariant;

        let mut variants = Vec::new();

        match &ast.data {
            Data::Enum(enum_) => {
                for (variant, var) in enum_.variants.iter().enumerate() {
                    variants.push(Struct::new(
                        StructParams {
                            variant: variant,
                            attrs: &var.attrs,
                            name: &var.ident,
                        },
                        &var.fields,
                    ));
                }
                data_variant = DataVariant::Enum;
            }
            Data::Struct(struct_) => {
                variants.push(Struct::new(
                    StructParams {
                        variant: 0,
                        attrs: &[],
                        name: name,
                    },
                    &struct_.fields,
                ));
                data_variant = DataVariant::Struct;
            }

            Data::Union(union_) => {
                let fields = Some(&union_.fields.named);
                let vari = Struct::with_fields(
                    StructParams {
                        variant: 0,
                        attrs: &[],
                        name: name,
                    },
                    fields,
                );
                variants.push(vari);
                data_variant = DataVariant::Union;
            }
        }

        Self {
            vis: &ast.vis,
            name,
            attrs: &ast.attrs,
            generics: &ast.generics,
            data_variant,
            variants,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataVariant {
    Struct,
    Enum,
    Union,
}

#[derive(Copy, Clone)]
pub struct FieldIndex {
    pub variant: usize,
    pub pos: usize,
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct StructParams<'a> {
    variant: usize,
    attrs: &'a [Attribute],
    name: &'a Ident,
}

/// A struct/union or a variant of an enum.
#[derive(Clone)]
pub struct Struct<'a> {
    /// The attributes of this `Struct`.
    ///
    /// If this is a struct/union:these is the same as DataStructure.attrs.
    ///
    /// If this is an enum:these are the attributes on the variant.
    pub attrs: &'a [Attribute],
    /// The name of this `Struct`.
    ///
    /// If this is a struct/union:these is the same as DataStructure.name.
    ///
    /// If this is an enum:this is the name of the variant.
    pub name: &'a Ident,
    pub fields: Vec<Field<'a>>,
    _priv: (),
}

impl<'a> Struct<'a> {
    fn new(p: StructParams<'a>, fields: &'a SynFields) -> Self {
        let fields = match fields {
            SynFields::Named(f) => Some(&f.named),
            SynFields::Unnamed(f) => Some(&f.unnamed),
            SynFields::Unit => None,
        };

        Self::with_fields(p, fields)
    }

    fn with_fields<I>(p: StructParams<'a>, fields: Option<I>) -> Self
    where
        I: IntoIterator<Item = &'a SynField>,
    {
        let fields = match fields {
            Some(x) => Field::from_iter(p, x),
            None => Vec::new(),
        };

        Self {
            attrs: p.attrs,
            name: p.name,
            fields,
            _priv: (),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Represent a struct field
///
#[derive(Clone)]
pub struct Field<'a> {
    pub index: FieldIndex,
    pub attrs: &'a [Attribute],
    /// identifier for the field,which is either an index(in a tuple struct) or a name.
    pub ident: FieldIdent<'a>,
    pub pattern_ident: Ident,
    pub ty: &'a Type,
}

impl<'a> Field<'a> {
    fn new(index: FieldIndex, field: &'a SynField) -> Self {
        let ident;
        let pattern_ident;
        match field.ident.as_ref() {
            Some(ident_) => {
                pattern_ident = format_ident!("f{}_7ac4rtizw8q", ident_);
                ident = FieldIdent::Named(ident_);
            }
            None => {
                let span = syn::spanned::Spanned::span(&field.ty);
                pattern_ident = Ident::new(&format!("f{}_7ac4rtizw8q", index.pos), span);
                ident = FieldIdent::new_index(index.pos);
            }
        };

        Self {
            index,
            attrs: &field.attrs,
            ident,
            pattern_ident,
            ty: &field.ty,
        }
    }

    /// Gets the identifier of this field as an `&Ident`.
    pub fn pattern_ident(&self) -> &Ident {
        &self.pattern_ident
    }

    fn from_iter<I>(p: StructParams<'a>, fields: I) -> Vec<Self>
    where
        I: IntoIterator<Item = &'a SynField>,
    {
        fields
            .into_iter()
            .enumerate()
            .map(|(pos, f)| {
                let fi = FieldIndex {
                    variant: p.variant,
                    pos,
                };
                Field::new(fi, f)
            })
            .collect()
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldIdent<'a> {
    Index(usize),
    Named(&'a Ident),
}

impl<'a> ToTokens for FieldIdent<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            FieldIdent::Index(ind) => syn::Index::from(ind).to_tokens(tokens),
            FieldIdent::Named(name) => name.to_tokens(tokens),
        }
    }
}

impl<'a> FieldIdent<'a> {
    fn new_index(index: usize) -> Self {
        FieldIdent::Index(index)
    }
}
