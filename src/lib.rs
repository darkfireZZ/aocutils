//! A collection of useful functions and types for solving [Advent of Code][aoc] puzzles.
//!
//! # Design choices
//!
//! ## Error Handling
//!
//! Errors in this crate are generally not handled. In most of the cases when something unexpected
//! happens, the library will panic. There is simply no need for elaborate error handling in
//! [Advent of Code][aoc]. In fact, you want quite the opposite. If there is an error, it most
//! likely indicates a bug in your code, in which case you probably want the code to fail instantly
//! and tell you what is wrong.
//!
//! ## Simplicity vs. Performance
//!
//! Whenever there is a tradeoff between simplicity and performance, this crate chooses simplicity
//! over performance. Performance does not generally matter too much in [Advent of Code][aoc]. On
//! the other hand, not having to deal with an unnecessarily complex API may prevent a bunch of
//! headaches.
//!
//! [aoc]: https://adventofcode.com

#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_debug)]
#![warn(missing_docs)]

mod grid;

pub use grid::Grid;

/// An extension trait that adds convenient functions on [`str`].
pub trait StrExt: private::Sealed {
    /// Returns an iterator over all disjoint contiguous sequences of ASCII digits, possibly
    /// preceded by a `'-'` sign.
    ///
    /// ```
    /// use aoclib::StrExt;
    ///
    /// let extracted: Vec<_> = "th1s w111 3xtr4ct 411 numb3rs".extract_numbers().collect();
    /// let result = ["1", "111", "3", "4", "411", "3"];
    /// assert_eq!(extracted, result);
    ///
    /// let with_sign: Vec<_> = "it also works with negative numbers: -458654-324"
    ///     .extract_numbers()
    ///     .collect();
    /// let result = ["-458654", "-324"];
    /// assert_eq!(with_sign, result);
    /// ```
    fn extract_numbers(&self) -> ExtractNumbers;
}

impl StrExt for str {
    fn extract_numbers(&self) -> ExtractNumbers {
        ExtractNumbers { remainder: self }
    }
}

/// The return type of [`StrExt::extract_numbers()`], see its documentation for more details.
pub struct ExtractNumbers<'a> {
    remainder: &'a str,
}

impl<'a> Iterator for ExtractNumbers<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remainder.find(|c: char| c.is_ascii_digit()) {
            Some(index) => {
                let start =
                    if (index > 0) && (self.remainder.as_bytes().get(index - 1) == Some(&b'-')) {
                        index - 1
                    } else {
                        index
                    };

                let rem_after_index = &self.remainder[(index + 1)..];
                let end = (index + 1)
                    + rem_after_index
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(rem_after_index.len());

                let return_value = &self.remainder[start..end];
                self.remainder = &self.remainder[end..];

                Some(return_value)
            }
            None => {
                self.remainder = "";
                None
            }
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl Sealed for str {}
}

#[cfg(test)]
mod tests {
    mod extract_numbers {
        use crate::StrExt;
        #[test]
        fn empty() {
            assert_eq!("".extract_numbers().next(), None);
        }

        #[test]
        fn no_number_gibberish() {
            let result = "afsdiufasndofasuefcvyy-yxcv<yofasoiehfavyx-<üdfijuanfhudsfasdfapcvive"
                .extract_numbers()
                .next();
            assert_eq!(result, None);
        }

        #[test]
        fn one() {
            let expected = ["1"];
            let actual: Vec<_> = "1".extract_numbers().collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn one_surrounded_by_gibberish() {
            let expected = ["1"];
            let actual: Vec<_> = "asdfiunfa$öifha1asdfubanvdualvne"
                .extract_numbers()
                .collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn minus_one() {
            let expected = ["-1"];
            let actual: Vec<_> = "-1".extract_numbers().collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn space_separated() {
            let expected = ["324", "-234", "83", "848", "-7", "11", "456789654345"];
            let actual: Vec<_> = "324 -234 83 848 -7 11 456789654345"
                .extract_numbers()
                .collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn sign_separated() {
            let expected = ["7", "-7", "-23", "123", "-56"];
            let actual: Vec<_> = "7-7-23+123-56".extract_numbers().collect();
            assert_eq!(actual, expected);
        }

        #[test]
        fn gibberish() {
            let expected = ["87", "32", "8", "2", "-3", "9", "9", "238", "37423", "-65"];
            let actual: Vec<_> =
                "asd87fb32asod8f2b-3brn9a9fzdnqp238ehqw37423rfasldfhasldhualksb-65faüe$spof"
                    .extract_numbers()
                    .collect();
            assert_eq!(actual, expected);
        }
    }
}
