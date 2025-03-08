use alloc::vec::Vec;

use bstr::ByteVec;

use crate::case_folding::CaseFoldingEffect;

/// Transform this UTF-8 buffer to "capitalized", returning a new `Vec<u8>`
/// and a [`CaseFoldingEffect`].
///
/// "Capitalized" here means:
///  - The **first** codepoint is converted to uppercase
///  - All subsequent codepoints are converted to lowercase
///
/// Invalid UTF‐8 bytes are passed through unchanged. If any valid codepoint
/// changes length or content, we mark [`CaseFoldingEffect::Changed`].
///
/// # Compatibility Notes
///
/// This function should use titlecase mapping for the initial character.
pub fn to_utf8_capitalized(mut bytes: &[u8]) -> (Vec<u8>, CaseFoldingEffect) {
    // This allocation assumes that in the common case, capitalizing and
    // lower-casing `char`s do not change the length of the `String`.
    //
    // Use a `Vec` here instead of a `Buf` to ensure at most one alloc fix-up
    // happens instead of alloc fix-ups being O(chars).
    let mut replacement = Vec::with_capacity(bytes.len());
    let mut effect = CaseFoldingEffect::Unchanged;

    let (ch, size) = bstr::decode_utf8(bytes);
    // SAFETY: bstr guarantees that the size is within the bounds of the slice.
    let (chunk, remainder) = unsafe { bytes.split_at_unchecked(size) };
    bytes = remainder;

    if let Some(ch) = ch {
        // Converting a UTF-8 character to uppercase may yield multiple
        // codepoints.
        let old = ch;
        for ch in ch.to_uppercase() {
            if ch != old {
                effect = CaseFoldingEffect::Modified;
            }
            replacement.push_char(ch);
        }
    } else {
        replacement.extend_from_slice(chunk);
    }

    while !bytes.is_empty() {
        let (ch, size) = bstr::decode_utf8(bytes);
        // SAFETY: bstr guarantees that the size is within the bounds of the slice.
        let (chunk, remainder) = unsafe { bytes.split_at_unchecked(size) };
        bytes = remainder;

        if let Some(ch) = ch {
            // Converting a UTF-8 character to lowercase may yield
            // multiple codepoints.
            let old = ch;
            for ch in ch.to_lowercase() {
                if ch != old {
                    effect = CaseFoldingEffect::Modified;
                }
                replacement.push_char(ch);
            }
        } else {
            replacement.extend_from_slice(chunk);
        }
    }

    (replacement, effect)
}

/// Transform this UTF-8 buffer to lowercase, returning a new `Vec<u8>` and a
/// [`CaseFoldingEffect`].
///
/// Invalid UTF‐8 bytes are passed through unchanged. If any valid codepoint
/// changes length or content, we mark [`CaseFoldingEffect::Changed`].
pub fn to_utf8_lowercase(mut bytes: &[u8]) -> (Vec<u8>, CaseFoldingEffect) {
    // This allocation assumes that in the common case, lower-casing `char`s do
    // not change the length of the `String`.
    //
    // Use a `Vec` here instead of a `Buf` to ensure at most one alloc fix-up
    // happens instead of alloc fix-ups being O(chars).
    let mut replacement = Vec::with_capacity(bytes.len());
    let mut effect = CaseFoldingEffect::Unchanged;

    while !bytes.is_empty() {
        // Decode the next UTF-8 codepoint
        let (ch, size) = bstr::decode_utf8(bytes);

        // SAFETY: bstr guarantees that the size is within the bounds of the slice.
        let (chunk, remainder) = unsafe { bytes.split_at_unchecked(size) };
        bytes = remainder;

        let Some(ch) = ch else {
            // Not valid UTF-8 at this position, so pass bytes through unchanged
            replacement.extend_from_slice(chunk);
            continue;
        };
        // Converting a UTF-8 character to lowercase may yield multiple
        // codepoints.
        let old = ch;
        for ch in old.to_lowercase() {
            if ch != old {
                effect = CaseFoldingEffect::Modified;
            }
            replacement.push_char(ch);
        }
    }

    (replacement, effect)
}

/// Transform this UTF-8 buffer to uppercase, returning a new `Vec<u8>` and a
/// [`CaseFoldingEffect`].
///
/// Invalid UTF‐8 bytes are passed through unchanged. If any valid codepoint
/// changes length or content, we mark [`CaseFoldingEffect::Changed`].
pub fn to_utf8_uppercase(mut bytes: &[u8]) -> (Vec<u8>, CaseFoldingEffect) {
    // This allocation assumes that in the common case, upper-casing `char`s do
    // not change the length of the `String`.
    //
    // Use a `Vec` here instead of a `Buf` to ensure at most one alloc fix-up
    // happens instead of alloc fix-ups being O(chars).
    let mut replacement = Vec::with_capacity(bytes.len());
    let mut effect = CaseFoldingEffect::Unchanged;

    while !bytes.is_empty() {
        // Decode the next UTF-8 codepoint
        let (ch, size) = bstr::decode_utf8(bytes);

        // SAFETY: bstr guarantees that the size is within the bounds of the slice.
        let (chunk, remainder) = unsafe { bytes.split_at_unchecked(size) };
        bytes = remainder;

        let Some(ch) = ch else {
            // Not valid UTF-8 at this position, so pass bytes through unchanged
            replacement.extend_from_slice(chunk);
            continue;
        };
        // Converting a UTF-8 character to lowercase may yield multiple
        // codepoints.
        let old = ch;
        for ch in old.to_uppercase() {
            if ch != old {
                effect = CaseFoldingEffect::Modified;
            }
            replacement.push_char(ch);
        }
    }

    (replacement, effect)
}

/// Transform this UTF-8 buffer to "swapped case", returning a new `Vec<u8>` and a
/// [`CaseFoldingEffect`].
///
/// Here "swapped case" means:
/// - Uppercase characters are converted to lowercase
/// - Lowercase characters are converted to uppercase
///
/// Invalid UTF‐8 bytes are passed through unchanged. If any valid codepoint
/// changes length or content, we mark [`CaseFoldingEffect::Changed`].
pub fn to_utf8_swapcase(mut bytes: &[u8]) -> (Vec<u8>, CaseFoldingEffect) {
    let mut replacement = Vec::with_capacity(bytes.len());
    let mut effect = CaseFoldingEffect::Unchanged;

    while !bytes.is_empty() {
        let (ch, size) = bstr::decode_utf8(bytes);
        // SAFETY: bstr guarantees `size` is in-bounds.
        let (chunk, remainder) = unsafe { bytes.split_at_unchecked(size) };
        bytes = remainder;

        let Some(ch) = ch else {
            // Not valid UTF-8 at this position, so pass bytes through
            // unchanged
            replacement.extend_from_slice(chunk);
            continue;
        };

        // If `ch` is uppercase, convert to lowercase; if lowercase, convert to
        // uppercase; otherwise, push as-is.
        //
        // FIXME: titlecase characters are not handled correctly.
        // FIXME: <https://github.com/artichoke/artichoke/issues/2834>
        match ch {
            old if old.is_lowercase() => {
                for ch in old.to_uppercase() {
                    if ch != old {
                        effect = CaseFoldingEffect::Modified;
                    }
                    replacement.push_char(ch);
                }
            }
            old if old.is_uppercase() => {
                for ch in old.to_lowercase() {
                    if ch != old {
                        effect = CaseFoldingEffect::Modified;
                    }
                    replacement.push_char(ch);
                }
            }
            old => replacement.push_char(old),
        }
    }

    (replacement, effect)
}

#[cfg(test)]
mod tests {

    use bstr::ByteSlice;

    use super::*;
    use crate::case_folding::CaseFoldingEffect;

    // Helper that runs a single test:
    #[track_caller]
    fn run_test<F>(func: F, input: &[u8], expected: &[u8], expect_fold: CaseFoldingEffect)
    where
        F: FnOnce(&[u8]) -> (Vec<u8>, CaseFoldingEffect),
    {
        let (output, effect) = func(input);
        assert_eq!(
            effect,
            expect_fold,
            "Expected folding effect {:?} for input {:?}, got {:?} with output {:?}",
            expect_fold,
            input.as_bstr(),
            effect,
            output.as_bstr(),
        );

        assert_eq!(
            output,
            expected,
            "Transformed result mismatch:\n  input = {:?}\n  expected = {:?}\n  actual   = {:?}",
            input.as_bstr(),
            expected.as_bstr(),
            output.as_bstr()
        );
    }

    #[test]
    fn test_to_utf8_capitalized() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 9] = [
            // 0) Empty
            (b"", b"", CaseFoldingEffect::Unchanged),
            // 1) ASCII
            (b"hello WORLD", b"Hello world", CaseFoldingEffect::Modified),
            (b"Hello world", b"Hello world", CaseFoldingEffect::Unchanged),
            (b"1234", b"1234", CaseFoldingEffect::Unchanged),
            // 2) partial invalid
            (b"\xFFabc", b"\xFFabc", CaseFoldingEffect::Unchanged),
            // 3) expansions with ß
            // 'ß' => upcase => "SS", 'test' => 'test' => "SStest"
            ("ßtest".as_bytes(), b"SStest", CaseFoldingEffect::Modified),
            // 4) Greek
            ("αγαπώ".as_bytes(), "Αγαπώ".as_bytes(), CaseFoldingEffect::Modified),
            // 5) Non-turkic folding mode for dotted i
            ("işaret".as_bytes(), "Işaret".as_bytes(), CaseFoldingEffect::Modified),
            // 6) Chinese
            (
                "你好世界".as_bytes(),
                "你好世界".as_bytes(),
                CaseFoldingEffect::Unchanged,
            ),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_capitalized, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_capitalized_dz_digraph() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 3] = [
            ("Ǆ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Unchanged),
            ("ǅ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Modified),
            ("ǆ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_capitalized, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_lowercase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 9] = [
            (b"", b"", CaseFoldingEffect::Unchanged),
            (b"HELLO", b"hello", CaseFoldingEffect::Modified),
            // 'ß' won't expand for lowercase, but 'T','E','S','T' => 't','e','s','t'
            ("ßTEST".as_bytes(), "ßtest".as_bytes(), CaseFoldingEffect::Modified),
            (b"\xFFhello", b"\xFFhello", CaseFoldingEffect::Unchanged),
            (b"Hello world", b"hello world", CaseFoldingEffect::Modified),
            (b"hello world", b"hello world", CaseFoldingEffect::Unchanged),
            // Turkish dotted I => 'İ' => 'i̇'
            ("İŞARET".as_bytes(), "i̇şaret".as_bytes(), CaseFoldingEffect::Modified),
            ("你好".as_bytes(), "你好".as_bytes(), CaseFoldingEffect::Unchanged),
            ("ΑΓΑΠΩ".as_bytes(), "αγαπω".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_lowercase, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_lowercase_dz_digraph() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 3] = [
            ("Ǆ".as_bytes(), "ǆ".as_bytes(), CaseFoldingEffect::Modified),
            ("ǅ".as_bytes(), "ǆ".as_bytes(), CaseFoldingEffect::Modified),
            ("ǆ".as_bytes(), "ǆ".as_bytes(), CaseFoldingEffect::Unchanged),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_lowercase, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_uppercase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 8] = [
            (b"", b"", CaseFoldingEffect::Unchanged),
            (b"hello", b"HELLO", CaseFoldingEffect::Modified),
            ("ßtest".as_bytes(), b"SSTEST", CaseFoldingEffect::Modified),
            (b"hello world", b"HELLO WORLD", CaseFoldingEffect::Modified),
            (b"HELLO", b"HELLO", CaseFoldingEffect::Unchanged),
            // Non-turkic folding mode for dotted i
            ("işaret".as_bytes(), "IŞARET".as_bytes(), CaseFoldingEffect::Modified),
            ("你好".as_bytes(), "你好".as_bytes(), CaseFoldingEffect::Unchanged),
            // Greek expansions
            ("αγαπώ".as_bytes(), "ΑΓΑΠΏ".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_uppercase, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_uppercase_dz_digraph() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 3] = [
            ("Ǆ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Unchanged),
            ("ǅ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Modified),
            ("ǆ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_uppercase, input, expected, effect);
        }
    }

    #[test]
    fn test_to_utf8_swapcase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 8] = [
            (b"", b"", CaseFoldingEffect::Unchanged),
            // ASCII
            (b"hEllo", b"HeLLO", CaseFoldingEffect::Modified),
            (b"1234", b"1234", CaseFoldingEffect::Unchanged),
            // expansions
            ("ßTEST".as_bytes(), "SStest".as_bytes(), CaseFoldingEffect::Modified),
            (b"\xFFabc", b"\xFFABC", CaseFoldingEffect::Modified),
            // Non-turkic folding mode for dotted i
            ("iŞARET".as_bytes(), "Işaret".as_bytes(), CaseFoldingEffect::Modified),
            // Chinese => no changes
            ("你好".as_bytes(), "你好".as_bytes(), CaseFoldingEffect::Unchanged),
            // Greek expansions
            ("αγΑΠΏ".as_bytes(), "ΑΓαπώ".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_swapcase, input, expected, effect);
        }
    }

    #[test]
    // currently lacking support for swapping the case of titlecase characters.
    // See: <https://github.com/artichoke/artichoke/issues/2834>
    #[should_panic = r#"Expected folding effect Modified for input "ǅ", got Unchanged with output "ǅ""#]
    fn test_to_utf8_swapcase_dz_digraph() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 4] = [
            ("Ǆ".as_bytes(), "ǆ".as_bytes(), CaseFoldingEffect::Modified),
            ("ǅ".as_bytes(), "dŽ".as_bytes(), CaseFoldingEffect::Modified),
            ("dŽ".as_bytes(), "Dž".as_bytes(), CaseFoldingEffect::Modified),
            ("ǆ".as_bytes(), "Ǆ".as_bytes(), CaseFoldingEffect::Modified),
        ];
        for (input, expected, effect) in cases {
            run_test(to_utf8_swapcase, input, expected, effect);
        }
    }
}
