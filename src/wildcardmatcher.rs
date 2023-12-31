pub struct Matcher {
  pattern: String,
}

impl Matcher {
  pub fn new(pattern: String) -> Self {
    return Matcher { pattern }
  }
  pub fn is_match(&self, s: String) -> bool {
    let mut i = 0;
    for character in self.pattern.chars() {
      match character {
        '?' => {
          if i < s.len() - 1 {
            i += 1;
            continue;
          }
          return false;
        },
        _ => {
          match s.chars().nth(i) {
            Some(userChar) => {
              if userChar == character {
                i += 1;
                continue;
              }
              return false;
            },
            None => {
              return false;
            }
          }
        }
      }
    }

    return i == s.len()
  }
}