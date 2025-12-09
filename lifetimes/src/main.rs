#![warn(missing_debug_implemenations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimeter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    } 

}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimeter) {
            let until_delimeter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimeter.len())..];
            Some(until_delimeter)
        }else if self.remainder.is_empty() {
            //Todo:bug
            None
        }else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]

fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, "");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}
