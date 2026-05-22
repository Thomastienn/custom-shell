pub type Prefix = String;

pub fn lcp(words: &[String]) -> Prefix {
    let mut i = 0;
    'outer: loop {
        for word in words {
            if i >= word.len() || word.as_bytes()[i] != words[0].as_bytes()[i] {
                break 'outer;
            }
        }
        i += 1;
    }
    words[0][..i].to_string()
}
