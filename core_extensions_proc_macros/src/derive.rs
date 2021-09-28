
pub(crate) mod const_default_derive;

pub(crate) mod datastructure;

pub(crate) mod utils;

#[allow(unused_imports)]
pub(crate) use self::{
    datastructure::{
        DataStructure,
        DataVariant,
        Field,
        FieldIdent,
        FieldIndex,
        Struct,
    },
    utils::{
        ParseBufferExt,
        SynResultExt,
    },
};
