/// What directions SliceExt::slice_lossy\[_mut\] is biased towards.
///
/// For `str` this has the effect of going in those directions if
/// the start/end bound is between char boundaries.
///
/// For `[T]` this has no effect and
/// it is recommended to use `()` as a parameter instead.
///
/// # Example
/// ```
/// use core_extensions::SliceExt;
/// use core_extensions::slices::SliceBias;
///
/// let word="niño";
///
/// assert_eq!(
///     word.char_indices().collect::<Vec<_>>(),
///     &[(0,'n'),(1,'i'),(2,'ñ'),(4,'o')]
/// );
///
/// assert_eq!(word.slice_lossy(0..1000      ,()            ),word);
/// assert_eq!(word.slice_lossy(10..10000    ,()            ),"");
/// assert_eq!(word.slice_lossy(0 ..4        ,()            ),"niñ");
/// assert_eq!(word.slice_lossy(0 ..3        ,()            ),"niñ");
/// assert_eq!(word.slice_lossy(0 ..2        ,()            ),"ni");
/// assert_eq!(word.slice_lossy(3 ..3        ,()            ),"ñ");
/// assert_eq!(word.slice_lossy(3 ..4        ,()            ),"ñ");
/// assert_eq!(word.slice_lossy(2 ..3        ,()            ),"ñ");
///
/// assert_eq!(word.slice_lossy(0..1000      ,SliceBias::OUT),word);
/// assert_eq!(word.slice_lossy(10..10000    ,SliceBias::OUT),"");
/// assert_eq!(word.slice_lossy(0 ..4        ,SliceBias::OUT),"niñ");
/// assert_eq!(word.slice_lossy(0 ..3        ,SliceBias::OUT),"niñ");
/// assert_eq!(word.slice_lossy(0 ..2        ,SliceBias::OUT),"ni");
/// assert_eq!(word.slice_lossy(3 ..3        ,SliceBias::OUT),"ñ");
/// assert_eq!(word.slice_lossy(3 ..4        ,SliceBias::OUT),"ñ");
/// assert_eq!(word.slice_lossy(2 ..3        ,SliceBias::OUT),"ñ");
///
/// assert_eq!(word.slice_lossy(0 ..10000    ,SliceBias::IN),word);
/// assert_eq!(word.slice_lossy(10..10000    ,SliceBias::IN),"");
/// assert_eq!(word.slice_lossy(0 ..4        ,SliceBias::IN),"niñ");
/// assert_eq!(word.slice_lossy(0 ..3        ,SliceBias::IN),"ni");
/// assert_eq!(word.slice_lossy(0 ..2        ,SliceBias::IN),"ni");
/// assert_eq!(word.slice_lossy(3 ..3        ,SliceBias::IN),"");
/// assert_eq!(word.slice_lossy(3 ..4        ,SliceBias::IN),"");
/// assert_eq!(word.slice_lossy(2 ..3        ,SliceBias::IN),"");
///
/// assert_eq!(word.slice_lossy(0..1000      ,SliceBias::LEFT),word);
/// assert_eq!(word.slice_lossy(10..10000    ,SliceBias::LEFT),"");
/// assert_eq!(word.slice_lossy(0 ..4        ,SliceBias::LEFT),"niñ");
/// assert_eq!(word.slice_lossy(0 ..3        ,SliceBias::LEFT),"ni");
/// assert_eq!(word.slice_lossy(0 ..2        ,SliceBias::LEFT),"ni");
/// assert_eq!(word.slice_lossy(3 ..3        ,SliceBias::LEFT),"");
/// assert_eq!(word.slice_lossy(3 ..4        ,SliceBias::LEFT),"ñ");
/// assert_eq!(word.slice_lossy(2 ..3        ,SliceBias::LEFT),"");
///
/// assert_eq!(word.slice_lossy(0..1000      ,SliceBias::RIGHT),word);
/// assert_eq!(word.slice_lossy(10..10000    ,SliceBias::RIGHT),"");
/// assert_eq!(word.slice_lossy(0 ..4        ,SliceBias::RIGHT),"niñ");
/// assert_eq!(word.slice_lossy(0 ..3        ,SliceBias::RIGHT),"niñ");
/// assert_eq!(word.slice_lossy(0 ..2        ,SliceBias::RIGHT),"ni");
/// assert_eq!(word.slice_lossy(3 ..3        ,SliceBias::RIGHT),"");
/// assert_eq!(word.slice_lossy(3 ..4        ,SliceBias::RIGHT),"");
/// assert_eq!(word.slice_lossy(2 ..3        ,SliceBias::RIGHT),"ñ");
///
///
///
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceBias {
    /// bias of the start bound
    pub start: BiasDirection,
    /// bias of the end bound
    pub end: BiasDirection,
}

/// The direction the range bound is biased towards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiasDirection {
    /// Means that the bound is biased to lower indices
    Left,
    /// Means that the bound is biased to higher indices
    Right,
}

impl SliceBias {
    /// Biased inwards,start bounds go right,end bounds go left.
    pub const IN: Self = Self {
        start: BiasDirection::Right,
        end: BiasDirection::Left,
    };
    /// Biased outwards,start bounds go left,end bounds go right.
    pub const OUT: Self = Self {
        start: BiasDirection::Left,
        end: BiasDirection::Right,
    };
    /// Biased leftwards,both bounds go left.
    pub const LEFT: Self = Self {
        start: BiasDirection::Left,
        end: BiasDirection::Left,
    };
    /// Biased rightwards.both bounds go right.
    pub const RIGHT: Self = Self {
        start: BiasDirection::Right,
        end: BiasDirection::Right,
    };
}

impl From<()> for SliceBias {
    fn from(_: ()) -> Self {
        Self::OUT
    }
}

impl From<BiasDirection> for SliceBias {
    fn from(dir: BiasDirection) -> Self {
        Self {
            start: dir,
            end: dir,
        }
    }
}

impl From<(BiasDirection,)> for SliceBias {
    fn from((dir,): (BiasDirection,)) -> Self {
        Self {
            start: dir,
            end: dir,
        }
    }
}

impl From<(BiasDirection, BiasDirection)> for SliceBias {
    fn from((start, end): (BiasDirection, BiasDirection)) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::{Rand, Rng};

    impl Rand for SliceBias {
        fn rand<R: Rng>(rng: &mut R) -> Self {
            fn rand_dir<R: Rng>(rng: &mut R) -> BiasDirection {
                match rng.gen_range::<u8>(0, 2) {
                    0 => BiasDirection::Left,
                    _=> BiasDirection::Right,
                }
            }

            SliceBias{
                start: rand_dir(rng),
                end: rand_dir(rng),
            }
        }
    }

    #[test]
    fn doc_comments() {
        use SliceExt;
        let word = "niño";

        assert_eq!(word.slice_lossy(0..1000, SliceBias::OUT), word);
        assert_eq!(word.slice_lossy(10..10000, SliceBias::OUT), "");
        assert_eq!(word.slice_lossy(0..4, SliceBias::OUT), "niñ");
        assert_eq!(word.slice_lossy(0..3, SliceBias::OUT), "niñ");
        assert_eq!(word.slice_lossy(0..2, SliceBias::OUT), "ni");

        assert_eq!(word.slice_lossy(0..10000, SliceBias::IN), word);
        assert_eq!(word.slice_lossy(10..10000, SliceBias::IN), "");
        assert_eq!(word.slice_lossy(0..4, SliceBias::IN), "niñ");
        assert_eq!(word.slice_lossy(0..3, SliceBias::IN), "ni");
        assert_eq!(word.slice_lossy(0..2, SliceBias::IN), "ni");
        assert_eq!(word.slice_lossy(3..3, SliceBias::IN), "");
        assert_eq!(word.slice_lossy(3..4, SliceBias::IN), "");
        assert_eq!(word.slice_lossy(2..3, SliceBias::IN), "");

        assert_eq!(word.slice_lossy(0..1000, SliceBias::LEFT), word);
        assert_eq!(word.slice_lossy(10..10000, SliceBias::LEFT), "");
        assert_eq!(word.slice_lossy(0..4, SliceBias::LEFT), "niñ");
        assert_eq!(word.slice_lossy(0..3, SliceBias::LEFT), "ni");
        assert_eq!(word.slice_lossy(0..2, SliceBias::LEFT), "ni");
        assert_eq!(word.slice_lossy(3..3, SliceBias::LEFT), "");
        assert_eq!(word.slice_lossy(3..4, SliceBias::LEFT), "ñ");
        assert_eq!(word.slice_lossy(2..3, SliceBias::LEFT), "");

        assert_eq!(word.slice_lossy(0..1000, SliceBias::RIGHT), word);
        assert_eq!(word.slice_lossy(10..10000, SliceBias::RIGHT), "");
        assert_eq!(word.slice_lossy(0..4, SliceBias::RIGHT), "niñ");
        assert_eq!(word.slice_lossy(0..3, SliceBias::RIGHT), "niñ");
        assert_eq!(word.slice_lossy(0..2, SliceBias::RIGHT), "ni");
        assert_eq!(word.slice_lossy(3..3, SliceBias::RIGHT), "");
        assert_eq!(word.slice_lossy(3..4, SliceBias::RIGHT), "");
        assert_eq!(word.slice_lossy(2..3, SliceBias::RIGHT), "ñ");
    }

}
