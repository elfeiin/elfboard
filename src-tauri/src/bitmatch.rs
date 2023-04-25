use std::ops::{BitAnd, Shr};

pub trait BitMatch {
    fn bitmatch(&self, bit_pattern: &str) -> bool;
}

impl BitMatch for [u8] {
    fn bitmatch(&self, bit_pattern: &str) -> bool {
        assert![!bit_pattern.is_empty()];
        assert![bit_pattern.is_ascii()];

        if bit_pattern == ".." {
            return true;
        }

        if bit_pattern.len() < self.len() * 8 {
            assert![bit_pattern.contains("..")]
        }

        let segments: Vec<&str> = bit_pattern.split("..").collect();
        assert![segments.len() <= 2];

        let mut checked = 0;

        for (i, segment) in segments.iter().enumerate() {
            assert![segment.len() <= bit_pattern.len() * 8];

            checked += segment.len();

            assert![checked <= self.len() * 8];

            if i == 0 {
                let bytes = segment.as_bytes();

                for (j, chunk) in bytes.chunks(8).enumerate() {
                    for (offset, b) in chunk.iter().enumerate() {
                        if *b == b'_' {
                            continue;
                        }
                        assert![*b == b'0' || *b == b'1'];
                        if self[j].shr(7 - offset).bitand(1) + b'0' != *b {
                            return false;
                        }
                    }
                }
            } else {
                let bytes = segment.as_bytes();

                for (j, chunk) in bytes.rchunks(8).enumerate() {
                    for (offset, b) in chunk.iter().rev().enumerate() {
                        if *b == b'_' {
                            continue;
                        }
                        assert![*b == b'0' || *b == b'1'];
                        if self[self.len() - (j + 1)].shr(offset).bitand(1) + b'0' != *b {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

macro_rules! impl_bitmatch {
    ($($typ:ident),*) => {
        $(impl BitMatch for $typ {
            fn bitmatch(&self, bit_pattern: &str) -> bool {
                self.to_be_bytes().bitmatch(bit_pattern)
            }
        })*
    };
}

impl_bitmatch! {i8, i16, i32, i64, i128, u8, u16, u32, u64, u128}

#[cfg(test)]
mod tests {
    use crate::bitmatch::BitMatch;

    #[test]
    fn low() {
        assert![1337u32.bitmatch("..10100111001")];
    }
    #[test]
    fn high() {
        assert![1337u32.bitmatch("00000000000000000000010100111001")]
    }
    #[test]
    fn all() {
        assert![1337u32.bitmatch("..")]
    }
    #[test]
    #[should_panic]
    fn none() {
        1337u32.bitmatch("");
    }
    #[test]
    #[should_panic]
    fn not_enough() {
        1337u32.bitmatch("0");
    }
    #[test]
    #[should_panic]
    fn too_many() {
        1337u32.bitmatch("0..1..01");
    }
}
