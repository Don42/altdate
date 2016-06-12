// This module were heavily influenced by rust-chrono by Kang Seonghoon
// chrono::format::strftime

use super::{Item, Fixed};

#[derive(Clone,Debug)]
pub struct StrftimeItems<'a> {
    remainder: &'a str,
    recons: &'static [Item<'static>],
}

impl<'a> StrftimeItems<'a> {
    pub fn new(s: &'a str) -> StrftimeItems<'a> {
        static FMT_NONE: [Item<'static>; 0] = [];
        StrftimeItems { remainder: s, recons: &FMT_NONE }
    }
}

impl<'a> Iterator for StrftimeItems<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Item<'a>> {
        if !self.recons.is_empty() {
            let item = self.recons[0];
            self.recons = &self.recons[1..];
            return Some(item);
        }

        match self.remainder.chars().next() {
            // we are done
            None => return None,

            // The next item is a specifier
            Some('%') => {
                self.remainder = &self.remainder[1..];

                macro_rules! next {
                    () => (
                        match self.remainder.chars().next() {
                            Some(x) => {
                                self.remainder = &self.remainder[x.len_utf8()..];
                                x
                            },
                            None => return Some(Item::Error), // premature end of string
                        }
                    )
                }

                let spec = next!();

                macro_rules! recons {
                    [$head:expr, $($tail:expr),+] => ({
                        const RECONS: &'static [Item<'static>] = &[$($tail),+];
                        self.recons = RECONS;
                        $head
                    })
                }

                let item = match spec {
                    'A' => fix!(LongWeekdayName),
                    _ => Item::Error,
                };
                Some(item)
            },

            Some(c) if c.is_whitespace() => {
                let nextspec = self.remainder.find(|c: char| !c.is_whitespace())
                                             .unwrap_or(self.remainder.len());
                assert!(nextspec > 0);
                let item = sp!(&self.remainder[..nextspec]);
                self.remainder = &self.remainder[nextspec..];
                Some(item)
            },

            // the next item is a literal
            _ => {
                let nextspec = self.remainder.find(|c: char| c.is_whitespace() || c == '%')
                                             .unwrap_or(self.remainder.len());
                assert!(nextspec > 0);
                let item = lit!(&self.remainder[..nextspec]);
                self.remainder = &self.remainder[nextspec..];
                Some(item)
            },
        }
    }
}
