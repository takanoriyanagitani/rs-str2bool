use std::io;

use io::ErrorKind;

pub trait AsciiByteToBool {
    type Error: std::error::Error;

    fn convert(&self, input: u8) -> Result<bool, Self::Error>;

    fn convert_lower(&self, input: u8) -> Result<bool, Self::Error> {
        self.convert(input.to_ascii_lowercase())
    }

    fn invalid_char2error(invalid_char: char) -> Self::Error;

    fn convert_ascii_char(&self, input: char) -> Result<bool, Self::Error> {
        let u: u8 = input
            .try_into()
            .map_err(|_| Self::invalid_char2error(input))?;
        self.convert(u)
    }

    fn convert_ascii_char_lower(&self, input: char) -> Result<bool, Self::Error> {
        self.convert_ascii_char(input.to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsciiByteToBoolPair {
    pub true_value: u8,
    pub false_value: u8,
}

impl Default for AsciiByteToBoolPair {
    fn default() -> Self {
        Self {
            true_value: b'1',
            false_value: b'0',
        }
    }
}

impl AsciiByteToBoolPair {
    pub fn new_yn() -> Self {
        Self {
            true_value: b'y',
            false_value: b'n',
        }
    }

    pub fn new_tf() -> Self {
        Self {
            true_value: b't',
            false_value: b'f',
        }
    }

    pub fn new_ox() -> Self {
        Self {
            true_value: b'o',
            false_value: b'x',
        }
    }

    pub fn new_custom(true_value: u8, false_value: u8) -> Self {
        Self {
            true_value,
            false_value,
        }
    }
}

impl AsciiByteToBoolPair {
    pub fn new_from_true_value(true_value: u8) -> Self {
        Self {
            true_value,
            false_value: 0,
        }
    }

    pub fn new_o() -> Self {
        Self::new_from_true_value(b'o')
    }

    pub fn new_o_capital() -> Self {
        Self::new_from_true_value(b'O')
    }

    pub fn new_x() -> Self {
        Self::new_from_true_value(b'x')
    }

    pub fn new_x_capital() -> Self {
        Self::new_from_true_value(b'X')
    }
}

impl AsciiByteToBoolPair {
    pub fn into_lower(self) -> Self {
        Self {
            true_value: self.true_value.to_ascii_lowercase(),
            false_value: self.false_value.to_ascii_lowercase(),
        }
    }

    pub fn into_upper(self) -> Self {
        Self {
            true_value: self.true_value.to_ascii_uppercase(),
            false_value: self.false_value.to_ascii_uppercase(),
        }
    }
}

impl AsciiByteToBool for AsciiByteToBoolPair {
    type Error = io::Error;

    fn invalid_char2error(invalid_char: char) -> Self::Error {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid boolean representation: {}", invalid_char),
        )
    }

    fn convert(&self, input: u8) -> Result<bool, Self::Error> {
        if input == self.true_value {
            Ok(true)
        } else if input == self.false_value {
            Ok(false)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid boolean representation",
            ))
        }
    }
}

#[cfg(test)]
mod ascii_byte_tests {
    use crate::ascii_byte::{AsciiByteToBool, AsciiByteToBoolPair};
    use std::io::{Error, ErrorKind};

    fn err_msg(err: &Error) -> String {
        err.to_string()
    }

    #[test]
    fn default_pair_converts_correctly() {
        let pair = AsciiByteToBoolPair::default();

        assert!(pair.convert(b'1').unwrap());
        assert!(!pair.convert(b'0').unwrap());
    }

    #[test]
    fn custom_pair_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_custom(b'y', b'n');

        assert!(pair.convert(b'y').unwrap());
        assert!(!pair.convert(b'n').unwrap());
    }

    #[test]
    fn yn_pair_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_yn();
        assert!(pair.convert(b'y').unwrap());
        assert!(!pair.convert(b'n').unwrap());
    }

    #[test]
    fn tf_pair_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_tf();
        assert!(pair.convert(b't').unwrap());
        assert!(!pair.convert(b'f').unwrap());
    }

    #[test]
    fn ox_pair_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_ox();
        assert!(pair.convert(b'o').unwrap());
        assert!(!pair.convert(b'x').unwrap());
    }

    #[test]
    fn into_lower_and_into_upper_work() {
        let pair = AsciiByteToBoolPair::new_tf();
        let lower = pair.into_lower();
        assert_eq!(lower.true_value, b't');
        assert_eq!(lower.false_value, b'f');

        let upper = pair.into_upper();
        assert_eq!(upper.true_value, b'T');
        assert_eq!(upper.false_value, b'F');
    }

    #[test]
    fn convert_lower_transforms_input() {
        let pair = AsciiByteToBoolPair::new_tf();

        assert!(pair.convert_lower(b't').unwrap());
        assert!(!pair.convert_lower(b'f').unwrap());

        assert!(pair.convert_lower(b'T').unwrap());
        assert!(!pair.convert_lower(b'F').unwrap());
    }

    #[test]
    fn convert_ascii_char_and_lower_work() {
        let pair = AsciiByteToBoolPair::new_yn();

        assert!(pair.convert_ascii_char('y').unwrap());
        assert!(!pair.convert_ascii_char('n').unwrap());

        assert!(pair.convert_ascii_char_lower('Y').unwrap());
        assert!(!pair.convert_ascii_char_lower('N').unwrap());
    }

    #[test]
    fn invalid_input_returns_error() {
        let pair = AsciiByteToBoolPair::default();

        let err = pair.convert(b'z').unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(err_msg(&err).contains("Invalid boolean representation"));

        let err_char = pair.convert_ascii_char('x').unwrap_err();
        assert_eq!(err_char.kind(), ErrorKind::InvalidInput);
        assert!(err_msg(&err_char).contains("Invalid boolean representation"));
    }

    #[test]
    fn new_from_true_value_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_from_true_value(b'o');
        assert!(pair.convert(b'o').unwrap());
        assert!(!pair.convert(0).unwrap());
        assert!(pair.convert(b'x').is_err());
    }

    #[test]
    fn new_o_is_equivalent_to_new_from_true_value() {
        let pair_from = AsciiByteToBoolPair::new_from_true_value(b'o');
        let pair_o = AsciiByteToBoolPair::new_o();

        assert_eq!(pair_from.true_value, pair_o.true_value);
        assert_eq!(pair_from.false_value, pair_o.false_value);

        assert!(pair_o.convert(b'o').unwrap());
        assert!(!pair_o.convert(0).unwrap());
        assert!(pair_o.convert(b'x').is_err());
    }

    #[test]
    fn new_o_capital_converts_only_capital() {
        let pair = AsciiByteToBoolPair::new_o_capital();

        assert!(pair.convert(b'O').unwrap());
        assert!(!pair.convert(0).unwrap());
        assert!(pair.convert(b'o').is_err());
        assert!(pair.convert(b'x').is_err());
    }

    #[test]
    fn new_x_converts_correctly() {
        let pair = AsciiByteToBoolPair::new_x();

        assert!(pair.convert(b'x').unwrap());
        assert!(!pair.convert(0).unwrap());
        assert!(pair.convert(b'O').is_err());
        assert!(pair.convert(b'o').is_err());
    }

    #[test]
    fn new_x_capital_converts_only_capital() {
        let pair = AsciiByteToBoolPair::new_x_capital();

        assert!(pair.convert(b'X').unwrap());
        assert!(!pair.convert(0).unwrap());
        assert!(pair.convert(b'x').is_err());
        assert!(pair.convert(b'O').is_err());
        assert!(pair.convert(b'o').is_err());
    }

    #[test]
    fn convert_ascii_char_and_lower_work_with_new_codes() {
        fn test_pair(pair: AsciiByteToBoolPair, c: char, expected: bool) {
            let res = pair.convert_ascii_char(c).unwrap();
            assert_eq!(res, expected);
        }

        let pair_o = AsciiByteToBoolPair::new_o();
        test_pair(pair_o, 'o', true);
        test_pair(pair_o, '\0', false);
        assert!(pair_o.convert_ascii_char('O').is_err());

        let pair_o_cap = AsciiByteToBoolPair::new_o_capital();
        test_pair(pair_o_cap, 'O', true);
        test_pair(pair_o_cap, '\0', false);
        assert!(pair_o_cap.convert_ascii_char('o').is_err());

        let pair_x = AsciiByteToBoolPair::new_x();
        test_pair(pair_x, 'x', true);
        test_pair(pair_x, '\0', false);
        assert!(pair_x.convert_ascii_char('X').is_err());

        let pair_x_cap = AsciiByteToBoolPair::new_x_capital();
        test_pair(pair_x_cap, 'X', true);
        test_pair(pair_x_cap, '\0', false);
        assert!(pair_x_cap.convert_ascii_char('x').is_err());
    }
}
