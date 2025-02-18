pub use private::SignedDecimal;

mod private {
    use crate::UnsignedDecimal;

    /// A signed version of [UnsignedDecimal]
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct SignedDecimal {
        value: UnsignedDecimal,
        // Invariant: negative must be false whenever value is 0
        negative: bool,
    }

    impl SignedDecimal {
        pub(crate) fn from_raw_value(value: UnsignedDecimal, negative: bool) -> Self {
            assert!(value.to_raw_value() != 0 || !negative);
            SignedDecimal { value, negative }
        }
        pub(crate) fn to_raw_value(&self) -> UnsignedDecimal {
            self.value
        }
        pub(crate) fn is_negative(&self) -> bool {
            self.negative
        }
    }
}
