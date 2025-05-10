mod parity;
pub use parity::{is_even as is_u16_even, is_odd as is_u16_odd};

pub trait Parity {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}
impl Parity for u16 {
    fn is_even(&self) -> bool {
        parity::is_even(*self)
    }

    fn is_odd(&self) -> bool {
        parity::is_odd(*self)
    }
}

impl Parity for u8 {
    fn is_even(&self) -> bool {
        parity::is_even(*self as u16)
    }

    fn is_odd(&self) -> bool {
        parity::is_odd(*self as u16)
    }
}

pub fn is_even<T: Parity>(value: T) -> bool {
    value.is_even()
}

pub fn is_odd<T: Parity>(value: T) -> bool {
    value.is_odd()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16() {
        assert!(is_even(0_u16));
        assert!(is_even(2_u16));
        assert!(is_even(65534_u16));

        assert!(is_odd(1_u16));
        assert!(is_odd(101_u16));
        assert!(is_odd(65535_u16));
    }

    #[test]
    fn test_u8() {
        assert!(is_even(0_u8));
        assert!(is_even(2_u8));
        assert!(is_even(254_u8));

        assert!(is_odd(1_u8));
        assert!(is_odd(101_u8));
        assert!(is_odd(255_u8));
    }
}

