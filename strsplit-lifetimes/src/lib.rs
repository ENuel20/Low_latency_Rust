//#![warn(missing_debug_implemenations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimeter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimeter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    } 

}

pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimeter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimeter.find_next(remainder) {
            let until_delimeter = &remainder[..delim_start];
            *remainder = &remainder[delim_end ..];
            Some(until_delimeter)
        }else {
            self.remainder.take()
        }
        
    }
}

impl Delimeter for &str {
    fn find_next(&self, s:&str) -> Option<(usize, usize)> {
        s.find(self).map(|start|(start, start + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s:&str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self)
            .map(|(start, _)|(start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c:char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn untill_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell")
}
#[test]

fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]

fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()))
}

