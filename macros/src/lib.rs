#[macro_use]
extern crate procedural_masquerade;
extern crate proc_macro;
#[cfg(test)]
#[macro_use]
extern crate quote;
extern crate syn;

use std::ascii::escape_default;
use std::ffi::CString;

define_proc_macros! {
    #[allow(non_snake_case)]
    pub fn cstr_internal__build_bytes(input: &str) -> String {
        let bytes = build_bytes(input);
        format!("const BYTES: &'static [u8] = {};", bytes)
    }
}

fn build_bytes(input: &str) -> String {
    let s = match syn::parse_str::<syn::LitStr>(input) {
        Ok(s) => s,
        _ => panic!("expected a string literal, got {}", input)
    };
    let cstr = match CString::new(s.value()) {
        Ok(s) => s,
        _ => panic!("literal must not contain zero char")
    };
    let mut bytes = Vec::new();
    bytes.extend(br#"b""#);
    bytes.extend(cstr.as_bytes().iter().flat_map(|&b| escape_default(b)));
    bytes.extend(br#"\0""#);
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::build_bytes;

    macro_rules! build_bytes {
        ($($t:tt)*) => {
            build_bytes(&quote!($($t)*).to_string())
        }
    }
    macro_rules! result {
        ($($t:tt)*) => {
            quote!($($t)*).to_string()
        }
    }

    #[test]
    fn test_build_bytes() {
        assert_eq!(build_bytes!("aaa"), result!(b"aaa\0"));
        assert_eq!(build_bytes!("\t\n\r\"\\'"), result!(b"\t\n\r\"\\\'\0"));
        assert_eq!(build_bytes!("\x01\x02 \x7f"), result!(b"\x01\x02 \x7f\0"));
        assert_eq!(build_bytes!("你好"), result!(b"\xe4\xbd\xa0\xe5\xa5\xbd\0"));
    }

    #[test]
    #[should_panic]
    fn test_build_bytes_nul_inside() {
        build_bytes!("a\x00a");
    }
}
