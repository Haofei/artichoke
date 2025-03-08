//! Shared encoding and decoding logic for binary and ASCII-8BIT strings.

use crate::case_folding::CaseFoldingEffect;

/// Convert this ASCII or binary string to a "capitalized" form in-place,
/// returning whether any changes were made.
///
/// This routine ensures the first character is upcased, while every subsequent
/// character is downcased. The function scans only until it detects the first
/// difference from the current capitalized form:
///
/// 1. If the very first byte differs once converted to uppercase, we mark
///    `CaseFoldingEffect::Changed`, downcase the remainder of the slice in one
///    go, and return immediately.
/// 2. If the first byte produces no change, the function delegates to
///    [`make_lowercase`] for the rest of the slice, again short-circuiting
///    once a difference is found.
///
/// If the entire slice is already in "capitalized" form, this function returns
/// [`CaseFoldingEffect::Unchanged`].
///
/// [`make_lowercase`]: crate::binascii::make_lowercase
pub fn make_capitalized(s: &mut [u8]) -> CaseFoldingEffect {
    // Capitalize means:
    //  1) Upcase the first character
    //  2) Downcase the rest
    //
    // Only do byte equality checks until the first difference is found.
    // If we find a difference on the first char, we mark `Changed` and
    // immediately do a no-check downcase on the remainder.
    // If we find no difference for the first char, then we proceed similarly
    // for the remainder with the same short-circuit logic.

    // If the buffer is empty => Unchanged
    let Some((first, s)) = s.split_first_mut() else {
        return CaseFoldingEffect::Unchanged;
    };

    // Upcase the first char
    let old_first = *first;
    let new_first = old_first.to_ascii_uppercase();
    *first = new_first;

    if old_first != new_first {
        // We found a difference in the first char. Now downcase the rest, no
        // further equality checks.
        s.make_ascii_lowercase();
        return CaseFoldingEffect::Modified;
    }

    // If we get here, the first char had no change. For the remainder of the
    // string, let's do a single-pass approach and delegate to
    // `make_lowercase()`.
    make_lowercase(s)
}

/// Convert this ASCII or binary string to lowercase in-place, returning
/// whether any changes were made.
///
/// The function scans each byte until it finds a character that actually
/// needs to be changed (e.g., `A` → `a`). Once we detect the first changed
/// byte, we call `make_ascii_lowercase()` on the rest of the slice and
/// immediately return `CaseFoldingEffect::Changed`.
///
/// If we reach the end of the string without finding any uppercase byte, we
/// return `CaseFoldingEffect::Unchanged`.
pub fn make_lowercase(mut s: &mut [u8]) -> CaseFoldingEffect {
    loop {
        // Split off the first byte. If there is no first byte (the slice is
        // empty), then we've scanned the entire string without encountering a
        // change, so we return `Unchanged`.
        let Some((head, tail)) = s.split_first_mut() else {
            return CaseFoldingEffect::Unchanged;
        };
        let old = *head;
        let new = old.to_ascii_lowercase();
        s = tail;

        // Overwrite this byte with its lowercase version.
        *head = new;

        // If this byte actually changed (e.g., old was 'A', new is 'a'), then
        // for the rest of the string we skip further comparisons and just do
        // the direct ASCII-lowercasing.
        if old != new {
            s.make_ascii_lowercase();
            return CaseFoldingEffect::Modified;
        }
    }
}

/// Convert this ASCII or binary string to uppercase in-place, returning
/// whether any changes were made.
///
/// The function scans each byte until it finds a character that actually
/// needs to be changed (e.g., `a` → `A`). Once we detect the first changed
/// byte, we call `make_ascii_uppercase()` on the rest of the slice and
/// immediately return `CaseFoldingEffect::Changed`.
///
/// If we reach the end of the string without finding any lowercase byte, we
/// return `CaseFoldingEffect::Unchanged`.
pub fn make_uppercase(mut s: &mut [u8]) -> CaseFoldingEffect {
    loop {
        let Some((head, tail)) = s.split_first_mut() else {
            return CaseFoldingEffect::Unchanged;
        };
        let old = *head;
        let new = old.to_ascii_uppercase();
        s = tail;

        *head = new;

        if old != new {
            s.make_ascii_uppercase();
            return CaseFoldingEffect::Modified;
        }
    }
}

/// Convert this ASCII or binary string to “swapcase” in-place, returning
/// whether any changes were made.
///
/// “Swapcase” means each ASCII-lowercase byte is converted to uppercase,
/// and each ASCII-uppercase byte is converted to lowercase; any other
/// byte is left unchanged. The algorithm short-circuits upon detecting
/// the first modified byte:
///
/// 1. It scans each byte, comparing the swapped version to the original.
/// 2. Once the first difference is found, the rest of the bytes are
///    “swapcased” without further equality checks, and
///    [`CaseFoldingEffect::Changed`] is returned.
/// 3. If the entire slice is processed with no changes, returns
///    [`CaseFoldingEffect::Unchanged`].
pub fn make_swapcase(mut s: &mut [u8]) -> CaseFoldingEffect {
    #[inline]
    fn to_swapcase(b: u8) -> u8 {
        if b.is_ascii_lowercase() {
            b.to_ascii_uppercase()
        } else if b.is_ascii_uppercase() {
            b.to_ascii_lowercase()
        } else {
            b
        }
    }

    loop {
        let Some((head, tail)) = s.split_first_mut() else {
            return CaseFoldingEffect::Unchanged;
        };
        let old = *head;
        let new = to_swapcase(old);
        s = tail;

        *head = new;

        if *head != old {
            // We found a difference => do a "no-check" swapcase for the rest
            for b in s {
                let old = *b;
                let new = to_swapcase(old);
                *b = new;
            }
            return CaseFoldingEffect::Modified;
        }
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::*;
    use crate::case_folding::CaseFoldingEffect;

    #[track_caller]
    fn run_test<F>(func: F, input: &[u8], expected: &[u8], expect_fold: CaseFoldingEffect)
    where
        F: FnOnce(&mut [u8]) -> CaseFoldingEffect,
    {
        let mut data = input.to_vec();
        let fold = func(&mut data);
        assert_eq!(fold, expect_fold, "CaseFoldingEffect mismatch on {:?}", input.as_bstr());
        assert_eq!(
            data.as_bstr(),
            expected.as_bstr(),
            "Result bytes mismatch on {:?}",
            input.as_bstr()
        );
    }

    // 1) make_capitalized
    #[test]
    fn test_make_capitalized() {
        // Each test scenario:
        // input, expected output, expected folding effect
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 7] = [
            // empty => no changes
            (b"", b"", CaseFoldingEffect::Unchanged),
            // numeric only => no letters to change
            (b"1234", b"1234", CaseFoldingEffect::Unchanged),
            // spaces, tabs => no letters
            (b" \t ", b" \t ", CaseFoldingEffect::Unchanged),
            // control chars
            (b"\x01\x02Hello", b"\x01\x02hello", CaseFoldingEffect::Modified),
            // Actually let's see: first char is \x01, upcase => \x01 no difference, so skip?
            // But second char is \x02 => no difference. Then 'H' -> 'H' (no difference?),
            // 'e' -> 'E'(difference). So short-circuit => rest all downcased => "Ello" => "ello"?
            // Actually we might want to carefully see how "make_capitalized" handles control chars at start...
            // We'll pick an example that changes after the first alpha:
            // We'll adapt next lines carefully or define it so it does or doesn't change.

            // invalid ASCII bytes
            (b"\xFFabc", b"\xFFabc", CaseFoldingEffect::Unchanged),
            // The first char \xFF is not changed by `to_ascii_uppercase()`,
            // so no difference => then 'a'->'A'(difference) => short-circuit => bc->"bc"?
            // => "Abc"? Wait carefully. We'll define the final.

            // We'll do simpler examples for clarity below:
            (b"hello world", b"Hello world", CaseFoldingEffect::Modified),
            (b"Hello", b"Hello", CaseFoldingEffect::Unchanged),
        ];

        for (input, expected, effect) in cases {
            run_test(make_capitalized, input, expected, effect);
        }
    }

    // 2) make_lowercase
    #[test]
    fn test_make_lowercase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 9] = [
            (b"", b"", CaseFoldingEffect::Unchanged),
            (b"1234", b"1234", CaseFoldingEffect::Unchanged),
            (b"HELLO", b"hello", CaseFoldingEffect::Modified),
            (b"Hello1", b"hello1", CaseFoldingEffect::Modified),
            // spaces + tab => no alpha
            (b"   \t", b"   \t", CaseFoldingEffect::Unchanged),
            // invalid ASCII
            (b"\x80\x81HI", b"\x80\x81hi", CaseFoldingEffect::Modified),
            // Greek bytes: they won't match ASCII 'A'..'Z', so no changes
            (
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                CaseFoldingEffect::Unchanged,
            ),
            // turkic string with dotted I => no changes in ASCII
            (b"\xc4\xb0 \xc4\xb1", b"\xc4\xb0 \xc4\xb1", CaseFoldingEffect::Unchanged),
            // Chinese => no changes
            (
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                CaseFoldingEffect::Unchanged,
            ),
        ];
        for (input, expected, effect) in cases {
            run_test(make_lowercase, input, expected, effect);
        }
    }

    // 3) make_uppercase
    #[test]
    fn test_make_uppercase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 9] = [
            (b"", b"", CaseFoldingEffect::Unchanged),
            (b"1234", b"1234", CaseFoldingEffect::Unchanged),
            (b"hello", b"HELLO", CaseFoldingEffect::Modified),
            (b"hEllo2", b"HELLO2", CaseFoldingEffect::Modified),
            (b"   \t", b"   \t", CaseFoldingEffect::Unchanged),
            // invalid ASCII
            (b"\x80\x81hi", b"\x80\x81HI", CaseFoldingEffect::Modified),
            // Greek => no ASCII changes
            (
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                CaseFoldingEffect::Unchanged,
            ),
            // turkic => no ASCII changes
            (b"\xc4\xb0 \xc4\xb1", b"\xc4\xb0 \xc4\xb1", CaseFoldingEffect::Unchanged),
            // Chinese => no changes
            (
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                CaseFoldingEffect::Unchanged,
            ),
        ];
        for (input, expected, effect) in cases {
            run_test(make_uppercase, input, expected, effect);
        }
    }

    // 4) make_swapcase
    #[test]
    fn test_make_swapcase() {
        let cases: [(&[u8], &[u8], CaseFoldingEffect); 10] = [
            // empty
            (b"", b"", CaseFoldingEffect::Unchanged),
            // numeric => no change
            (b"123", b"123", CaseFoldingEffect::Unchanged),
            // ASCII letters
            (b"hEllO", b"HeLLo", CaseFoldingEffect::Modified),
            (b"HELLO", b"hello", CaseFoldingEffect::Modified),
            (b"hello", b"HELLO", CaseFoldingEffect::Modified),
            // spaces => no alpha
            (b"   \t", b"   \t", CaseFoldingEffect::Unchanged),
            // invalid ASCII
            (b"\xffAB", b"\xffab", CaseFoldingEffect::Modified),
            // Greek => no ASCII changes
            (
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                b"\xce\x93\xce\xb5\xce\xb9\xce\xac",
                CaseFoldingEffect::Unchanged,
            ),
            // turkic => no ASCII changes
            (b"\xc4\xb0 \xc4\xb1", b"\xc4\xb0 \xc4\xb1", CaseFoldingEffect::Unchanged),
            // Chinese => no changes
            (
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                b"\xe4\xbd\xa0\xe5\xa5\xbd",
                CaseFoldingEffect::Unchanged,
            ),
        ];
        for (input, expected, effect) in cases {
            run_test(make_swapcase, input, expected, effect);
        }
    }
}
