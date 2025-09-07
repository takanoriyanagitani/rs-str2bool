use std::io;

use io::ErrorKind;

pub trait AsciiBytesToBool {
    type Error: std::error::Error;

    fn convert(&self, input: &[u8]) -> Result<bool, Self::Error>;
}

pub struct AsciiBytesToBoolPair {
    pub true_value: &'static [u8],
    pub false_value: &'static [u8],
}

impl Default for AsciiBytesToBoolPair {
    fn default() -> Self {
        Self {
            true_value: b"true",
            false_value: b"false",
        }
    }
}

impl AsciiBytesToBoolPair {
    pub fn new_yes_no() -> Self {
        Self {
            true_value: b"yes",
            false_value: b"no",
        }
    }

    pub fn new_y_n() -> Self {
        Self {
            true_value: b"y",
            false_value: b"n",
        }
    }

    pub fn new_o_x() -> Self {
        Self {
            true_value: b"o",
            false_value: b"x",
        }
    }

    pub fn new_t_f() -> Self {
        Self {
            true_value: b"t",
            false_value: b"f",
        }
    }

    pub fn new_on_off() -> Self {
        Self {
            true_value: b"on",
            false_value: b"off",
        }
    }

    pub fn new_yes_no_capitalised() -> Self {
        Self {
            true_value: b"Yes",
            false_value: b"No",
        }
    }

    pub fn new_on_off_capitalised() -> Self {
        Self {
            true_value: b"On",
            false_value: b"Off",
        }
    }

    pub fn new_true_false_capitalised() -> Self {
        Self {
            true_value: b"True",
            false_value: b"False",
        }
    }

    pub fn new_true_false() -> Self {
        Self {
            true_value: b"true",
            false_value: b"false",
        }
    }

    pub fn new_custom(true_value: &'static [u8], false_value: &'static [u8]) -> Self {
        Self {
            true_value,
            false_value,
        }
    }
}

impl AsciiBytesToBoolPair {
    pub fn new_from_true_value(tv: &'static [u8]) -> Self {
        Self {
            true_value: tv,
            false_value: b"",
        }
    }

    pub fn new_o() -> Self {
        Self::new_from_true_value(b"o")
    }

    pub fn new_o_capital() -> Self {
        Self::new_from_true_value(b"O")
    }

    pub fn new_x() -> Self {
        Self::new_from_true_value(b"x")
    }

    pub fn new_x_capital() -> Self {
        Self::new_from_true_value(b"X")
    }
}

impl AsciiBytesToBool for AsciiBytesToBoolPair {
    type Error = io::Error;

    fn convert(&self, input: &[u8]) -> Result<bool, Self::Error> {
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
mod tests {
    use super::*;
    use std::io::ErrorKind;

    fn err_msg(err: &std::io::Error) -> String {
        err.to_string()
    }

    #[test]
    fn default_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::default();

        assert!(pair.convert(b"true").unwrap());
        assert!(!pair.convert(b"false").unwrap());
        let err = pair.convert(b"").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn yes_no_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_yes_no();
        assert!(pair.convert(b"yes").unwrap());
        assert!(!pair.convert(b"no").unwrap());
    }

    #[test]
    fn y_n_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_y_n();
        assert!(pair.convert(b"y").unwrap());
        assert!(!pair.convert(b"n").unwrap());
    }

    #[test]
    fn o_x_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_o_x();
        assert!(pair.convert(b"o").unwrap());
        assert!(!pair.convert(b"x").unwrap());
    }

    #[test]
    fn t_f_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_t_f();
        assert!(pair.convert(b"t").unwrap());
        assert!(!pair.convert(b"f").unwrap());
    }

    #[test]
    fn on_off_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_on_off();
        assert!(pair.convert(b"on").unwrap());
        assert!(!pair.convert(b"off").unwrap());
    }

    #[test]
    fn capitalised_pairs_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_yes_no_capitalised();
        assert!(pair.convert(b"Yes").unwrap());
        assert!(!pair.convert(b"No").unwrap());

        let pair = AsciiBytesToBoolPair::new_on_off_capitalised();
        assert!(pair.convert(b"On").unwrap());
        assert!(!pair.convert(b"Off").unwrap());

        let pair = AsciiBytesToBoolPair::new_true_false_capitalised();
        assert!(pair.convert(b"True").unwrap());
        assert!(!pair.convert(b"False").unwrap());
    }

    #[test]
    fn custom_pair_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_custom(b"ok", b"nope");
        assert!(pair.convert(b"ok").unwrap());
        assert!(!pair.convert(b"nope").unwrap());
    }

    #[test]
    fn invalid_input_returns_error() {
        let pair = AsciiBytesToBoolPair::default();

        let err = pair.convert(b"maybe").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(err_msg(&err).contains("Invalid boolean representation"));

        let err = pair.convert(b"truest").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn different_length_inputs_are_rejected() {
        let pair = AsciiBytesToBoolPair::new_t_f();
        assert!(pair.convert(b"t").is_ok());
        assert!(pair.convert(b"tt").is_err());
    }

    #[test]
    fn null_slice_returns_error() {
        let pair = AsciiBytesToBoolPair::default();
        let err = pair.convert(&[]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn new_from_true_value_converts_correctly() {
        let pair = AsciiBytesToBoolPair::new_from_true_value(b"o");

        assert!(pair.convert(b"o").unwrap());

        assert!(!pair.convert(b"").unwrap());

        let err = pair.convert(b"x").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn new_o_is_equivalent_to_new_from_true_value() {
        let pair_from = AsciiBytesToBoolPair::new_from_true_value(b"o");
        let pair_o = AsciiBytesToBoolPair::new_o();

        assert_eq!(pair_from.true_value, pair_o.true_value);
        assert_eq!(pair_from.false_value, pair_o.false_value);

        assert!(pair_o.convert(b"o").unwrap());
        assert!(!pair_o.convert(b"").unwrap());
    }

    #[test]
    fn new_o_capital() {
        let pair = AsciiBytesToBoolPair::new_o_capital();

        assert!(pair.convert(b"O").unwrap());

        let err = pair.convert(b"o").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);

        assert!(!pair.convert(b"").unwrap());
    }

    #[test]
    fn new_x() {
        let pair = AsciiBytesToBoolPair::new_x();

        assert!(pair.convert(b"x").unwrap());
        assert!(!pair.convert(b"").unwrap());

        let err = pair.convert(b"y").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn new_x_capital() {
        let pair = AsciiBytesToBoolPair::new_x_capital();

        assert!(pair.convert(b"X").unwrap());
        assert!(!pair.convert(b"").unwrap());

        let err = pair.convert(b"x").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }
}
