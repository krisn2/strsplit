//! 
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    remainder:&'a str, // the str reference with lifetime of <'a>
    delimiter:&'a str, // the str reference with lifetime of <'a>
}
impl StrSplit <'_> {
    pub new(haystack:&str, delimiter:&str){
        Self{
            remainder:haystack,
            delimiter,
        }
    }
}

impl Iterator<'a> for StrSplit<'a> {
    type Item = &str;
    fn next (&mut self)-> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter);
        let until_delimiter = &self.remainder[..next_delim];
        self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
    } else if self.remainder.is_empty() {
        None
    }else {
        let rest = self.remainder;
        self.remainder = & [];
        Some(rest)
    }

}


#[test]
fn it_work() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a","b", "c", "d", "e"].into_iter());
}
