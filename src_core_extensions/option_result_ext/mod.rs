//! Contains \[extension\] traits for Option and Result

mod result_like;

mod option_extensions;

mod result_extensions;

#[doc(inline)]
pub use self::option_extensions::*;
#[doc(inline)]
pub use self::result_extensions::*;
#[doc(inline)]
pub use self::result_like::ResultLike;
