#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    // TODO strings of different length?
    const STR: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    // fn consume_iterator<T>(mut iter: Iterator<Item = T>) {
    //     for item in iter {
    //         black_box(item);
    //     }
    // }
    #[bench]
    // fn regular_split_consume_words(b: &mut Bencher) {
    fn regular_split_iter_over_bytes(b: &mut Bencher) {
        b.iter(|| {
            for word in STR.split_whitespace() {
                for byte in word.bytes() {
                    black_box(byte);
                }
            }
        })
    }
    #[bench]
    fn regular_split_iter_over_chars(b: &mut Bencher) {
        b.iter(|| {
            for word in STR.split_whitespace() {
                for c in word.chars() {
                    black_box(c);
                }
            }
        });
    }
    #[bench]
    fn regular_split_dont_read_word(b: &mut Bencher) {
        b.iter(|| {
            for word in STR.split_whitespace() {
                black_box(word);
            }
        });
    }
}
