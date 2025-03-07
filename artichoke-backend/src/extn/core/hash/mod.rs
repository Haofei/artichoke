use std::ffi::CStr;

use crate::extn::prelude::*;

const HASH_CSTR: &CStr = c"Hash";
static HASH_RUBY_SOURCE: &[u8] = include_bytes!("hash.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Hash>() {
        return Ok(());
    }

    let spec = class::Spec::new("Hash", HASH_CSTR, None, None)?;
    interp.def_class::<Hash>(spec)?;
    interp.eval(HASH_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Hash;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Hash";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("hash_functional_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }

    #[test]
    fn regression_github_1099() {
        let mut interp = interpreter();
        let inspect = interp.eval(b"{ a: 'GH-1099' }.inspect").unwrap();
        let inspect = inspect.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(inspect, r#"{:a=>"GH-1099"}"#);
    }
}
