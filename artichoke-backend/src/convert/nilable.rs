//! Converters for nilable primitive Ruby types.
//!
//! Excludes collection types Array and Hash.

use crate::Artichoke;
use crate::core::{Convert, TryConvert, TryConvertMut, Value as _};
use crate::error::Error;
use crate::value::Value;

impl Convert<Option<Value>, Value> for Artichoke {
    fn convert(&self, value: Option<Value>) -> Value {
        Value::from(value)
    }
}

impl Convert<Option<i64>, Value> for Artichoke {
    fn convert(&self, value: Option<i64>) -> Value {
        let Some(value) = value else {
            return Value::nil();
        };
        self.convert(value)
    }
}

impl TryConvert<Option<usize>, Value> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Option<usize>) -> Result<Value, Self::Error> {
        let Some(value) = value else {
            return Ok(Value::nil());
        };
        self.try_convert(value)
    }
}

impl TryConvertMut<Option<Vec<u8>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<Vec<u8>>) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.as_deref())
    }
}

impl TryConvertMut<Option<&[u8]>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<&[u8]>) -> Result<Value, Self::Error> {
        let Some(value) = value else {
            return Ok(Value::nil());
        };
        self.try_convert_mut(value)
    }
}

impl TryConvertMut<Option<String>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<String>) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.as_deref())
    }
}

impl TryConvertMut<Option<&str>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<&str>) -> Result<Value, Self::Error> {
        let Some(value) = value else {
            return Ok(Value::nil());
        };
        self.try_convert_mut(value)
    }
}

impl Convert<Value, Option<Value>> for Artichoke {
    fn convert(&self, value: Value) -> Option<Value> {
        if value.is_nil() { None } else { Some(value) }
    }
}

impl TryConvertMut<Value, Option<Vec<u8>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<Vec<u8>>, Self::Error> {
        if value.is_nil() {
            return Ok(None);
        }
        self.try_convert_mut(value).map(Some)
    }
}

impl<'a> TryConvertMut<Value, Option<&'a [u8]>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<&'a [u8]>, Self::Error> {
        if value.is_nil() {
            return Ok(None);
        }
        self.try_convert_mut(value).map(Some)
    }
}

impl TryConvertMut<Value, Option<String>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<String>, Self::Error> {
        if value.is_nil() {
            return Ok(None);
        }
        self.try_convert_mut(value).map(Some)
    }
}

impl<'a> TryConvertMut<Value, Option<&'a str>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<&'a str>, Self::Error> {
        if value.is_nil() {
            return Ok(None);
        }
        self.try_convert_mut(value).map(Some)
    }
}

impl TryConvert<Value, Option<i64>> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Value) -> Result<Option<i64>, Self::Error> {
        if value.is_nil() {
            return Ok(None);
        }
        self.try_convert(value).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Convert, TryConvert, TryConvertMut};
    use crate::test::prelude::*;
    use crate::value::Value;

    #[test]
    fn convert_option_value_some_returns_the_inner_value() {
        let interp = interpreter();
        let inner = interp.convert(42_i64);
        let value = interp.convert(Some(inner));
        // Ensure the converted value is not nil.
        assert!(!value.is_nil());
        // Roundtrip: convert back to i64.
        let num = value.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(num, 42);
    }

    #[test]
    fn convert_option_value_none_returns_nil() {
        let interp = interpreter();
        let value = interp.convert(None::<Value>);
        assert!(value.is_nil());
    }

    #[test]
    fn convert_value_to_option_value_non_nil_returns_some() {
        let interp = interpreter();
        let value = interp.convert(789_i64);
        let opt: Option<Value> = interp.convert(value);
        assert!(opt.is_some());
        let num = opt.unwrap().try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(num, 789);
    }

    #[test]
    fn convert_value_to_option_value_nil_returns_none() {
        let interp = interpreter();
        let nil_value = Value::nil();
        let opt: Option<Value> = interp.convert(nil_value);
        assert!(opt.is_none());
    }

    // --- Tests for Option<i64> -> Value ---

    #[test]
    fn convert_option_i64_some_converts_to_fixnum() {
        let interp = interpreter();
        let value = interp.convert(Some(123_i64));
        let num = value.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(num, 123);
    }

    #[test]
    fn convert_option_i64_none_converts_to_nil() {
        let interp = interpreter();
        let value = interp.convert(None::<i64>);
        assert!(value.is_nil());
    }

    // --- Tests for Option<usize> -> Value via TryConvert ---

    #[test]
    fn try_convert_option_usize_some_converts_to_fixnum() {
        let interp = interpreter();
        let value = interp.try_convert(Some(456_usize)).unwrap();
        let num: usize = value.try_convert_into(&interp).unwrap();
        assert_eq!(num, 456);
    }

    #[test]
    fn try_convert_option_usize_none_converts_to_nil() {
        let interp = interpreter();
        let value = interp.try_convert(None::<usize>).unwrap();
        assert!(value.is_nil());
    }

    // --- Tests for Option<Vec<u8>> -> Value via TryConvertMut ---

    #[test]
    fn try_convert_mut_option_vec_u8_some_converts_to_string() {
        let mut interp = interpreter();
        let input = b"hello".to_vec();
        let value = interp.try_convert_mut(Some(input.clone())).unwrap();
        // Convert the resulting Ruby value back to String.
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert_eq!(result, Some(String::from("hello")));
    }

    #[test]
    fn try_convert_mut_option_vec_u8_none_converts_to_nil() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut(None::<Vec<u8>>).unwrap();
        assert!(value.is_nil());
    }

    // --- Tests for Option<&[u8]> -> Value via TryConvertMut ---

    #[test]
    fn try_convert_mut_option_slice_u8_some_converts_to_string() {
        let mut interp = interpreter();
        let input: &[u8] = b"world";
        let value = interp.try_convert_mut(Some(input)).unwrap();
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert_eq!(result, Some(String::from("world")));
    }

    #[test]
    fn try_convert_mut_option_slice_u8_none_converts_to_nil() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut(None::<&[u8]>).unwrap();
        assert!(value.is_nil());
    }

    #[test]
    fn try_convert_mut_option_string_some_converts_to_string() {
        let mut interp = interpreter();
        let input = String::from("artichoke");
        let value = interp.try_convert_mut(Some(input.clone())).unwrap();
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert_eq!(result, Some(input));
    }

    #[test]
    fn try_convert_mut_option_string_none_converts_to_nil() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut(None::<String>).unwrap();
        assert!(value.is_nil());
    }

    // --- Tests for Option<&str> -> Value via TryConvertMut ---

    #[test]
    fn try_convert_mut_option_str_some_converts_to_string() {
        let mut interp = interpreter();
        let input = "convert me";
        let value = interp.try_convert_mut(Some(input)).unwrap();
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert_eq!(result, Some(input.to_owned()));
    }

    #[test]
    fn try_convert_mut_option_str_none_converts_to_nil() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut(None::<&str>).unwrap();
        assert!(value.is_nil());
    }

    // --- Tests for Value -> Option<Vec<u8>> via TryConvertMut ---

    #[test]
    fn try_convert_mut_value_to_option_vec_u8_non_nil_returns_some() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut("hello world").unwrap();
        let result = value.try_convert_into_mut::<Option<Vec<u8>>>(&mut interp).unwrap();
        let s = String::from_utf8(result.unwrap()).unwrap();
        assert_eq!(s, "hello world");
    }

    #[test]
    fn try_convert_mut_value_to_option_vec_u8_nil_returns_none() {
        let mut interp = interpreter();
        let value = Value::nil();
        let result = value.try_convert_into_mut::<Option<Vec<u8>>>(&mut interp).unwrap();
        assert!(result.is_none());
    }

    // --- Tests for Value -> Option<&[u8]> via TryConvertMut ---

    #[test]
    fn try_convert_mut_value_to_option_slice_u8_non_nil_returns_some() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut("slice-test").unwrap();
        let result = value.try_convert_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
        let s = String::from_utf8(result.unwrap().to_vec()).unwrap();
        assert_eq!(s, "slice-test");
    }

    #[test]
    fn try_convert_mut_value_to_option_slice_u8_nil_returns_none() {
        let mut interp = interpreter();
        let value = Value::nil();
        let result = value.try_convert_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
        assert!(result.is_none());
    }

    // --- Tests for Value -> Option<String> via TryConvertMut ---

    #[test]
    fn try_convert_mut_value_to_option_string_non_nil_returns_some() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut("string-test").unwrap();
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert_eq!(result, Some(String::from("string-test")));
    }

    #[test]
    fn try_convert_mut_value_to_option_string_nil_returns_none() {
        let mut interp = interpreter();
        let value = Value::nil();
        let result = value.try_convert_into_mut::<Option<String>>(&mut interp).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn try_convert_mut_value_to_option_str_non_nil_returns_some() {
        let mut interp = interpreter();
        let value = interp.try_convert_mut("str-test").unwrap();
        let result = value.try_convert_into_mut::<Option<&str>>(&mut interp).unwrap();
        assert_eq!(result, Some("str-test"));
    }

    #[test]
    fn try_convert_mut_value_to_option_str_nil_returns_none() {
        let mut interp = interpreter();
        let value = Value::nil();
        let result = value.try_convert_into_mut::<Option<&str>>(&mut interp).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn try_convert_value_to_option_i64_non_nil_returns_some() {
        let interp = interpreter();
        let value = interp.convert(555i64);
        let result = value.try_convert_into::<Option<i64>>(&interp).unwrap();
        assert_eq!(result, Some(555));
    }

    #[test]
    fn try_convert_value_to_option_i64_nil_returns_none() {
        let interp = interpreter();
        let value = Value::nil();
        let result = value.try_convert_into::<Option<i64>>(&interp).unwrap();
        assert!(result.is_none());
    }
}
