use std::str::Chars;

pub trait SplitStringLazy {
    fn split_whitespace_lazy(&self) -> ();
}

pub struct SplitWhitespaceLazy<'a> {
    chars_iter: std::str::Chars<'a>,
    last_checked_char: char,
    last_checked_char_is_part_of_word: bool,
}

// TODO how about this "inherits" std::str::Split? So methods like "last" are inherited?
// Or am I speaking gibberish?
impl<'a> SplitWhitespaceLazy<'_> {
    fn next_when_last_part_is_not_consumed(&self) {

    }
    fn next_when_last_part_is_consumed(&self) {

    }
}

// Ietrate over word's chars
// TODO how do we make it NOT ask for the `SplitWhitespaceLazy` iterator?
// `Rc<>`, `RefCell`? Yeah, `RefCell` seems appropriate because we're mutating
// the state of the `SplitWhitespaceLazy` struct, but its `impl Iterator for SplitWhitespaceLazy`
// appears to act exactly the same (except for performance).
fn next(self, split_whitespace_lazy: SplitWhitespaceLazy) {
    // TODO turn this pseudo-pseudo-code into real code.
    let next = split_whitespace_lazy.last_checked_char;
    split_whitespace_lazy.last_checked_char = split_whitespace_lazy.chars_iter.next();
    return next;
}

impl<'a> Iterator for SplitWhitespaceLazy<'a> {
    type Item = std::str::Chars<'a>;
    fn next(&mut self) -> Option<Chars<'a>> {
        // self.chars = self.next_when_last_part_is_consumed;
        if self.last_checked_char_is_part_of_word {
            // Find the end of the current word.

            // TODO perf: since we know the end of the current word, we can update
            // the current word's iterator so it doesn't have to check for whitespace
            // but instead just this end position.
        }

        // TODO how about this function?
        // self.chars_iter = self.chars_iter.skip_while(predicate);


        // At this `curr_char` is between the words (at the first char after the current word).
        // Find the start of the new word.
        // let next_word_first_char = for ch in self.chars_iter {
        let next_word_first_char = loop {
            // let next_char = match self.chars_iter.next() {
            self.last_checked_char = match self.chars_iter.next() {
                None => {
                    // TODO modify the curr word iterator as well.
                    return None;
                },
                Some(ch) => ch,
            };
            // TODO different filters.
            if !self.last_checked_char.is_whitespace() {
                break;
                // break self.last_checked_char;
            }
        };

        // TODO here we're supposed to watch 
        let next_word_iter = std::iter::once(self.last_checked_char)
            .chain(self.chars_iter.take_while(|ch| !ch.is_whitespace()));
        Some(next_word_iter)

        // "TODO".bytes().next().unwrap().str
        // "TODO".as
        // let test = "test";
        // let b = test.as_bytes();
        // drop(test);
        // drop(test);
    }
}

// impl str {
impl SplitStringLazy for str {
    fn split_whitespace_lazy(&self) /* -> Iterator<Item = Iterator<Item = char>> */ {
        let chars = self.chars();
        // chars.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::SplitStringLazy;

    #[test]
    fn it_works() {
        let test_str = "  abc def   gh\nijkl  mnop ";
        let words_normal = test_str.split_whitespace();
        let words_my = test_str.split_whitespace_lazy();
        // TODO assert they have the same length (so not `zip`?).
        for (word_normal, word_my) in words_normal.zip(words_my) {
            for (ch_normal, ch_my) in word_normal.zip(word_my) {
                assert_eq!(ch_normal, ch_my);
            }
        }
    }
}
