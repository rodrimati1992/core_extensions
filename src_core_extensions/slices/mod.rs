//! Slice extension traits, and related items.

mod extensions;
mod slice_bias;
mod slice_split_while;
pub use self::extensions::{ValSliceExt,SliceExt};
pub use self::slice_bias::BiasDirection;
pub use self::slice_bias::SliceBias;
pub use self::slice_split_while::{KeySlice,SplitSliceWhile,RSplitSliceWhile};
