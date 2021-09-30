/// Derives the [`TransparentNewtype`] trait.
/// [`TransparentNewtype`]: ./transparent_newtype/trait.TransparentNewtype.html
#[cfg_attr(feature = "docsrs", doc(cfg(all(feature = "derive", feature = "transparent_newtype"))))]
pub use core_extensions_proc_macros::TransparentNewtype;