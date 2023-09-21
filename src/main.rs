mod wildcardmatcher;

use wildcardmatcher::Matcher;

fn main() {
    let matcher = Matcher::new("abc?e".to_string());
    println!("{}", matcher.is_match("abcde".to_string()));
}
