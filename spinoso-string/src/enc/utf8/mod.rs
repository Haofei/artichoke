macro_rules! impl_partial_eq {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_bytes(), other)
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_bytes())
            }
        }
    };
}

macro_rules! impl_partial_eq_array {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b, const N: usize> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_bytes(), other)
            }
        }

        impl<'a, 'b, const N: usize> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_bytes())
            }
        }
    };
}

mod borrowed;
mod case_change;
mod inspect;
mod owned;

pub use borrowed::Codepoints;
pub use borrowed::Utf8Str;
pub use inspect::Inspect;
pub use owned::Utf8String;

#[cfg(test)]
#[expect(clippy::invisible_characters, reason = "testing naughty UTF-8 strings")]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::str;

    use super::{Utf8Str, Utf8String};
    use crate::test::run_arbitrary;

    const REPLACEMENT_CHARACTER_BYTES: [u8; 3] = [239, 191, 189];

    #[test]
    fn prop_fuzz_char_len_utf8_contents_utf8_string() {
        run_arbitrary::<String>(|contents| {
            let expected = contents.chars().count();
            let s = Utf8String::from(contents);
            assert_eq!(s.char_len(), expected);
        });
    }

    #[test]
    fn prop_fuzz_len_utf8_contents_utf8_string() {
        run_arbitrary::<String>(|contents| {
            let expected = contents.len();
            let s = Utf8String::from(contents);
            assert_eq!(s.len(), expected);
        });
    }

    #[test]
    fn prop_fuzz_char_len_binary_contents_utf8_string() {
        run_arbitrary::<Vec<u8>>(|contents| {
            if let Ok(utf8_contents) = str::from_utf8(&contents) {
                let expected = utf8_contents.chars().count();
                let s = Utf8String::from(contents);
                assert_eq!(s.char_len(), expected);
            } else {
                let expected_at_most = contents.len();
                let s = Utf8String::from(contents);
                assert!(s.char_len() <= expected_at_most);
            }
        });
    }

    #[test]
    fn prop_fuzz_len_binary_contents_utf8_string() {
        run_arbitrary::<Vec<u8>>(|contents| {
            let expected = contents.len();
            let s = Utf8String::from(contents);
            assert_eq!(s.len(), expected);
        });
    }

    #[test]
    fn constructs_empty_buffer() {
        let s = Utf8String::from(Vec::new());
        assert_eq!(0, s.len());
    }

    #[test]
    fn char_len_empty() {
        let s = Utf8String::from("");
        assert_eq!(s.char_len(), 0);
    }

    #[test]
    fn char_len_ascii() {
        let s = Utf8String::from("Artichoke Ruby");
        assert_eq!(s.char_len(), 14);
    }

    #[test]
    fn char_len_emoji() {
        let s = Utf8String::from("💎");
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from("💎🦀🎉");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("a💎b🦀c🎉d");
        assert_eq!(s.char_len(), 7);
        // with invalid UTF-8 bytes
        let s = Utf8String::from(b"a\xF0\x9F\x92\x8E\xFFabc");
        assert_eq!(s.char_len(), 6);
    }

    #[test]
    fn char_len_unicode_replacement_character() {
        let s = Utf8String::from("�");
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from("���");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("a�b�c�d");
        assert_eq!(s.char_len(), 7);
        let s = Utf8String::from("�💎b🦀c🎉�");
        assert_eq!(s.char_len(), 7);
        // with invalid UFF-8 bytes
        let s = Utf8String::from(b"\xEF\xBF\xBD\xF0\x9F\x92\x8E\xFF\xEF\xBF\xBDab");
        assert_eq!(s.char_len(), 6);
        let s = Utf8String::from(REPLACEMENT_CHARACTER_BYTES);
        assert_eq!(s.char_len(), 1);
    }

    #[test]
    fn char_len_nul_byte() {
        let s = Utf8String::from(b"\x00");
        assert_eq!(s.char_len(), 1);
        let s = Utf8String::from(b"abc\x00");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from(b"abc\x00xyz");
        assert_eq!(s.char_len(), 7);
    }

    #[test]
    fn char_len_invalid_utf8_byte_sequences() {
        let s = Utf8String::from(b"\x00\x00\xD8\x00");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from(b"\xFF\xFE");
        assert_eq!(s.char_len(), 2);
    }

    #[test]
    fn char_len_binary() {
        let bytes = &[
            0xB3, 0x7E, 0x39, 0x70, 0x8E, 0xFD, 0xBB, 0x75, 0x62, 0x77, 0xE7, 0xDF, 0x6F, 0xF2, 0x76, 0x27, 0x81,
            0x9A, 0x3A, 0x9D, 0xED, 0x6B, 0x4F, 0xAE, 0xC4, 0xE7, 0xA1, 0x66, 0x11, 0xF1, 0x08, 0x1C,
        ];
        let s = Utf8String::from(bytes);
        assert_eq!(s.char_len(), 32);
        // Mixed binary and ASCII
        let bytes = &[
            b'?', b'!', b'a', b'b', b'c', 0xFD, 0xBB, 0x75, 0x62, 0x77, 0xE7, 0xDF, 0x6F, 0xF2, 0x76, 0x27, 0x81,
            0x9A, 0x3A, 0x9D, 0xED, 0x6B, 0x4F, 0xAE, 0xC4, 0xE7, 0xA1, 0x66, 0x11, 0xF1, 0x08, 0x1C,
        ];
        let s = Utf8String::from(bytes);
        assert_eq!(s.char_len(), 32);
    }

    #[test]
    fn char_len_mixed_ascii_emoji_invalid_bytes() {
        // ```
        // [2.6.3] > s = "🦀abc💎\xff"
        // => "🦀abc💎\xFF"
        // [2.6.3] > s.length
        // => 6
        // [2.6.3] > puts s.bytes.map{|b| "\\x#{b.to_s(16).upcase}"}.join
        // \xF0\x9F\xA6\x80\x61\x62\x63\xF0\x9F\x92\x8E\xFF
        // ```
        let s = Utf8String::from(b"\xF0\x9F\xA6\x80\x61\x62\x63\xF0\x9F\x92\x8E\xFF");
        assert_eq!(s.char_len(), 6);
    }

    #[test]
    fn char_len_utf8() {
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L147-L157
        let s = Utf8String::from("Ω≈ç√∫˜µ≤≥÷");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("åß∂ƒ©˙∆˚¬…æ");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("œ∑´®†¥¨ˆøπ“‘");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("¡™£¢∞§¶•ªº–≠");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("¸˛Ç◊ı˜Â¯˘¿");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("ÅÍÎÏ˝ÓÔÒÚÆ☃");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("Œ„´‰ˇÁ¨ˆØ∏”’");
        assert_eq!(s.char_len(), 12);
        let s = Utf8String::from("`⁄€‹›ﬁﬂ‡°·‚—±");
        assert_eq!(s.char_len(), 13);
        let s = Utf8String::from("⅛⅜⅝⅞");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from("ЁЂЃЄЅІЇЈЉЊЋЌЍЎЏАБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя");
        assert_eq!(s.char_len(), 79);
    }

    #[test]
    fn char_len_vmware_super_string() {
        // A super string recommended by VMware Inc. Globalization Team: can
        // effectively cause rendering issues or character-length issues to
        // validate product globalization readiness.
        //
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L202-L224
        let s = Utf8String::from("表ポあA鷗ŒéＢ逍Üßªąñ丂㐀𠀀");
        assert_eq!(s.char_len(), 17);
    }

    #[test]
    fn char_len_two_byte_chars() {
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L188-L196
        let s = Utf8String::from("田中さんにあげて下さい");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("パーティーへ行かないか");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("和製漢語");
        assert_eq!(s.char_len(), 4);
        let s = Utf8String::from("部落格");
        assert_eq!(s.char_len(), 3);
        let s = Utf8String::from("사회과학원 어학연구소");
        assert_eq!(s.char_len(), 11);
        let s = Utf8String::from("찦차를 타고 온 펲시맨과 쑛다리 똠방각하");
        assert_eq!(s.char_len(), 22);
        let s = Utf8String::from("社會科學院語學研究所");
        assert_eq!(s.char_len(), 10);
        let s = Utf8String::from("울란바토르");
        assert_eq!(s.char_len(), 5);
        let s = Utf8String::from("𠜎𠜱𠝹𠱓𠱸𠲖𠳏");
        assert_eq!(s.char_len(), 7);
    }

    #[test]
    fn char_len_space_chars() {
        // Whitespace: all the characters with category `Zs`, `Zl`, or `Zp` (in Unicode
        // version 8.0.0), plus `U+0009 (HT)`, `U+000B (VT)`, `U+000C (FF)`, `U+0085 (NEL)`,
        // and `U+200B` (ZERO WIDTH SPACE), which are in the C categories but are often
        // treated as whitespace in some contexts.
        //
        // This file unfortunately cannot express strings containing
        // `U+0000`, `U+000A`, or `U+000D` (`NUL`, `LF`, `CR`).
        //
        // The next line may appear to be blank or mojibake in some viewers.
        //
        // The next line may be flagged for "trailing whitespace" in some viewers.
        //
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L131
        let bytes = "	              ​    　
";
        let s = Utf8String::from(bytes);
        assert_eq!(s.char_len(), 25);
    }

    #[test]
    fn casing_utf8_string_empty() {
        let mut s = Utf8String::from(b"");

        s.make_capitalized();
        assert_eq!(s, "");

        s.make_lowercase();
        assert_eq!(s, "");

        s.make_uppercase();
        assert_eq!(s, "");

        s.make_swapcase();
        assert_eq!(s, "");
    }

    #[test]
    fn casing_utf8_string_ascii() {
        let lower = Utf8String::from(b"abc");
        let mid_upper = Utf8String::from(b"aBc");
        let upper = Utf8String::from(b"ABC");
        let long = Utf8String::from(b"aBC, 123, ABC, baby you and me girl");

        let capitalize: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };
        let swapcase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_swapcase();
            value
        };

        assert_eq!(capitalize(&lower), "Abc");
        assert_eq!(capitalize(&mid_upper), "Abc");
        assert_eq!(capitalize(&upper), "Abc");
        assert_eq!(capitalize(&long), "Abc, 123, abc, baby you and me girl");

        assert_eq!(lowercase(&lower), "abc");
        assert_eq!(lowercase(&mid_upper), "abc");
        assert_eq!(lowercase(&upper), "abc");
        assert_eq!(lowercase(&long), "abc, 123, abc, baby you and me girl");

        assert_eq!(uppercase(&lower), "ABC");
        assert_eq!(uppercase(&mid_upper), "ABC");
        assert_eq!(uppercase(&upper), "ABC");
        assert_eq!(uppercase(&long), "ABC, 123, ABC, BABY YOU AND ME GIRL");

        assert_eq!(swapcase(&lower), "ABC");
        assert_eq!(swapcase(&mid_upper), "AbC");
        assert_eq!(swapcase(&upper), "abc");
        assert_eq!(swapcase(&long), "Abc, 123, abc, BABY YOU AND ME GIRL");
    }

    #[test]
    fn casing_utf8_string_utf8() {
        // Capitalization of `ß` (SS) differs from MRI:
        //
        // ```console
        // [2.6.3] > "ß".capitalize
        // => "Ss"
        // ```
        let sharp_s = Utf8String::from("ß");
        let tomorrow = Utf8String::from("αύριο");
        let year = Utf8String::from("έτος");
        // two-byte characters
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L198-L200
        let two_byte_chars = Utf8String::from("𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐑂 𐑄 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆");
        // Changes length when case changes
        // https://github.com/minimaxir/big-list-of-naughty-strings/blob/894882e7/blns.txt#L226-L232
        let varying_length = Utf8String::from("zȺȾ");
        // There doesn't appear to be any RTL scripts that have cases, but might as well make sure
        let rtl = Utf8String::from("مرحبا الخرشوف");

        let capitalize: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_capitalized();
            value
        };
        let lowercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_lowercase();
            value
        };
        let uppercase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_uppercase();
            value
        };
        let swapcase: fn(&Utf8String) -> Utf8String = |value: &Utf8String| {
            let mut value = value.clone();
            value.make_swapcase();
            value
        };

        assert_eq!(capitalize(&sharp_s), "SS");
        assert_eq!(capitalize(&tomorrow), "Αύριο");
        assert_eq!(capitalize(&year), "Έτος");
        assert_eq!(
            capitalize(&two_byte_chars),
            "𐐜 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐑁𐐲𐑉𐑅𐐻/𐑅𐐯𐐿𐐲𐑌𐐼 𐐺𐐳𐐿 𐐺𐐴 𐑄 𐑉𐐨𐐾𐐯𐑌𐐻𐑅 𐐱𐑂 𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐐷𐐮𐐭𐑌𐐮𐑂𐐲𐑉𐑅𐐮𐐻𐐮"
        );
        assert_eq!(capitalize(&varying_length), "Zⱥⱦ");
        assert_eq!(capitalize(&rtl), "مرحبا الخرشوف");

        assert_eq!(lowercase(&sharp_s), "ß");
        assert_eq!(lowercase(&tomorrow), "αύριο");
        assert_eq!(lowercase(&year), "έτος");
        assert_eq!(
            lowercase(&two_byte_chars),
            "𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐑁𐐲𐑉𐑅𐐻/𐑅𐐯𐐿𐐲𐑌𐐼 𐐺𐐳𐐿 𐐺𐐴 𐑄 𐑉𐐨𐐾𐐯𐑌𐐻𐑅 𐐱𐑂 𐑄 𐐼𐐯𐑅𐐨𐑉𐐯𐐻 𐐷𐐮𐐭𐑌𐐮𐑂𐐲𐑉𐑅𐐮𐐻𐐮"
        );
        assert_eq!(lowercase(&varying_length), "zⱥⱦ");
        assert_eq!(lowercase(&rtl), "مرحبا الخرشوف");

        assert_eq!(uppercase(&sharp_s), "SS");
        assert_eq!(uppercase(&tomorrow), "ΑΎΡΙΟ");
        assert_eq!(uppercase(&year), "ΈΤΟΣ");
        assert_eq!(
            uppercase(&two_byte_chars),
            "𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐙𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐒𐐋𐐗 𐐒𐐌 𐐜 𐐡𐐀𐐖𐐇𐐤𐐓𐐝 𐐉𐐚 𐐜 𐐔𐐇𐐝𐐀𐐡𐐇𐐓 𐐏𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(uppercase(&varying_length), "ZȺȾ");
        assert_eq!(uppercase(&rtl), "مرحبا الخرشوف");

        let sharp_s = Utf8String::from("SS");
        let tomorrow = Utf8String::from("Αύριο");
        let year = Utf8String::from("Έτος");
        // This next line is the titlecase version of the earlier two-byte string:
        let two_byte_chars = Utf8String::from("𐐜 𐐔𐐯𐑅𐐨𐑉𐐯𐐻 𐐙𐐲𐑉𐑅𐐻/𐑅𐐯𐐿𐐲𐑌𐐼 𐐒𐐳𐐿 𐐒𐐴 𐐜 𐐡𐐨𐐾𐐯𐑌𐐻𐑅 𐐉𐑂 𐐜 𐐔𐐯𐑅𐐨𐑉𐐯𐐻 𐐏𐐮𐐭𐑌𐐮𐑂𐐲𐑉𐑅𐐮𐐻𐐮");
        let varying_length = Utf8String::from("Zⱥⱦ");
        let rtl = Utf8String::from("مرحبا الخرشوف");
        assert_eq!(swapcase(&sharp_s), "ss", "swapcase(SS) failed");
        assert_eq!(swapcase(&tomorrow), "αΎΡΙΟ", "swapcase(Αύριο) failed");
        assert_eq!(swapcase(&year), "έΤΟΣ", "swapcase(Έτος) failed");
        assert_eq!(
            swapcase(&two_byte_chars),
            "𐑄 𐐼𐐇𐐝𐐀𐐡𐐇𐐓 𐑁𐐊𐐡𐐝𐐓/𐐝𐐇𐐗𐐊𐐤𐐔 𐐺𐐋𐐗 𐐺𐐌 𐑄 𐑉𐐀𐐖𐐇𐐤𐐓𐐝 𐐱𐐚 𐑄 𐐼𐐇𐐝𐐀𐐡𐐇𐐓 𐐷𐐆𐐅𐐤𐐆𐐚𐐊𐐡𐐝𐐆𐐓𐐆"
        );
        assert_eq!(swapcase(&varying_length), "zȺȾ", "swapcase(Zⱥⱦ) mismatch");
        assert_eq!(swapcase(&rtl), "مرحبا الخرشوف", "swapcase(مرحبا الخرشوف) mismatch");
    }

    #[test]
    fn casing_utf8_string_invalid_utf8() {
        let mut s = Utf8String::from(b"\xFF\xFE");

        s.make_capitalized();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_lowercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_uppercase();
        assert_eq!(s, &b"\xFF\xFE"[..]);

        s.make_swapcase();
        assert_eq!(s, &b"\xFF\xFE"[..]);
    }

    #[test]
    fn casing_utf8_string_unicode_replacement_character() {
        let mut s = Utf8String::from("�");

        s.make_capitalized();
        assert_eq!(s, "�");

        s.make_lowercase();
        assert_eq!(s, "�");

        s.make_uppercase();
        assert_eq!(s, "�");

        s.make_swapcase();
        assert_eq!(s, "�");
    }

    #[test]
    fn chr_does_not_return_more_than_one_byte_for_invalid_utf8() {
        // ```ruby
        // [3.0.1] > "\xF0\x9F\x87".chr
        // => "\xF0"
        // ```
        //
        // Per `bstr`:
        //
        // The bytes `\xF0\x9F\x87` could lead to a valid UTF-8 sequence, but 3 of them
        // on their own are invalid. Only one replacement codepoint is substituted,
        // which demonstrates the "substitution of maximal subparts" strategy.
        let s = Utf8String::from(b"\xF0\x9F\x87");
        assert_eq!(s.chr(), b"\xF0");
    }

    #[test]
    fn get_char_slice_valid_range() {
        let s = Utf8String::from(b"a\xF0\x9F\x92\x8E\xFF".to_vec()); // `"a💎\xFF"`
        assert_eq!(s.get_char_slice(0..0), Some(Utf8Str::empty()));
        assert_eq!(s.get_char_slice(0..1), Some(Utf8Str::new(b"a")));
        assert_eq!(s.get_char_slice(0..2), Some(Utf8Str::new("a💎")));
        assert_eq!(s.get_char_slice(0..3), Some(Utf8Str::new(b"a\xF0\x9F\x92\x8E\xFF")));
        assert_eq!(s.get_char_slice(0..4), Some(Utf8Str::new(b"a\xF0\x9F\x92\x8E\xFF")));
        assert_eq!(s.get_char_slice(1..1), Some(Utf8Str::empty()));
        assert_eq!(s.get_char_slice(1..2), Some(Utf8Str::new("💎")));
        assert_eq!(s.get_char_slice(1..3), Some(Utf8Str::new(b"\xF0\x9F\x92\x8E\xFF")));
    }

    #[test]
    #[expect(clippy::reversed_empty_ranges, reason = "testing behavior of reversed ranges")]
    fn get_char_slice_invalid_range() {
        let s = Utf8String::from(b"a\xF0\x9F\x92\x8E\xFF".to_vec()); // `"a💎\xFF"`
        assert_eq!(s.get_char_slice(4..5), None);
        assert_eq!(s.get_char_slice(4..1), None);
        assert_eq!(s.get_char_slice(3..1), Some(Utf8Str::empty()));
        assert_eq!(s.get_char_slice(2..1), Some(Utf8Str::empty()));
        assert_eq!(s.get_char_slice(7..10), None);
        assert_eq!(s.get_char_slice(10..8), None);
        assert_eq!(s.get_char_slice(10..5), None);
        assert_eq!(s.get_char_slice(10..2), None);
    }

    #[test]
    fn index_with_default_offset() {
        let s = Utf8String::from("f💎oo");
        assert_eq!(s.index("f".as_bytes(), 0), Some(0));
        assert_eq!(s.index("o".as_bytes(), 0), Some(2));
        assert_eq!(s.index("oo".as_bytes(), 0), Some(2));
        assert_eq!(s.index("ooo".as_bytes(), 0), None);
    }

    #[test]
    fn index_with_different_offset() {
        let s = Utf8String::from("f💎oo");
        assert_eq!(s.index("o".as_bytes(), 1), Some(2));
        assert_eq!(s.index("o".as_bytes(), 2), Some(2));
        assert_eq!(s.index("o".as_bytes(), 3), Some(3));
        assert_eq!(s.index("o".as_bytes(), 4), None);
    }

    #[test]
    fn rindex_with_default_offset() {
        let s = Utf8String::from("f💎oo");
        assert_eq!(s.rindex("f".as_bytes(), 3), Some(0));
        assert_eq!(s.rindex("o".as_bytes(), 3), Some(3));
        assert_eq!(s.rindex("oo".as_bytes(), 3), Some(2));
        assert_eq!(s.rindex("ooo".as_bytes(), 3), None);
    }

    #[test]
    fn rindex_with_different_offset() {
        let s = Utf8String::from("f💎oo");
        assert_eq!(s.rindex("o".as_bytes(), 4), Some(3));
        assert_eq!(s.rindex("o".as_bytes(), 3), Some(3));
        assert_eq!(s.rindex("o".as_bytes(), 2), Some(2));
        assert_eq!(s.rindex("o".as_bytes(), 1), None);
        assert_eq!(s.rindex("o".as_bytes(), 0), None);
    }

    #[test]
    fn index_and_rindex_support_invalid_utf8_in_needle() {
        // Invalid UTF-8 in needle
        let needle = &"💎".as_bytes()[..3];

        assert_eq!(Utf8String::from("f💎oo").index(needle, 0), None); // FIXME: Currently `Some(1)`
        assert_eq!(Utf8String::from("f💎oo").rindex(needle, 3), None); // FIXME: Currently `Some(1)`
    }

    #[test]
    fn index_and_rindex_support_invalid_utf8_in_haystack() {
        // Invalid UTF-8 in haystack
        let mut haystack = Vec::new();
        haystack.extend_from_slice(b"f");
        haystack.extend_from_slice(&"💎".as_bytes()[..2]);
        haystack.extend_from_slice(b"oo");
        let haystack = Utf8String::from(haystack);

        assert_eq!(haystack.index("💎".as_bytes(), 0), None);
        assert_eq!(haystack.rindex("💎".as_bytes(), 3), None);
    }

    #[test]
    fn index_empties() {
        // ```console
        // [3.2.2] > "".index ""
        // => 0
        // [3.2.2] > "".index "a"
        // => nil
        // [3.2.2] > "a".index ""
        // => 0
        // ```
        let s = Utf8String::from("");
        assert_eq!(s.index(b"", 0), Some(0));

        assert_eq!(s.index(b"a", 0), None);

        let s = Utf8String::from("a");
        assert_eq!(s.index(b"", 0), Some(0));
    }

    #[test]
    fn rindex_empties() {
        // ```console
        // [3.2.2] > "".rindex ""
        // => 0
        // [3.2.2] > "".rindex "a"
        // => nil
        // [3.2.2] > "a".rindex ""
        // => 1
        // ```
        let s = Utf8String::from("");
        assert_eq!(s.rindex(b"", usize::MAX), Some(0));
        assert_eq!(s.rindex(b"", 1), Some(0));
        assert_eq!(s.rindex(b"", 0), Some(0));

        assert_eq!(s.rindex(b"a", usize::MAX), None);
        assert_eq!(s.rindex(b"a", 1), None);
        assert_eq!(s.rindex(b"a", 0), None);

        let s = Utf8String::from("a");
        assert_eq!(s.rindex(b"", usize::MAX), Some(1));
        assert_eq!(s.rindex(b"", 1), Some(1));
    }
}
