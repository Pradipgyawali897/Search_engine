pub struct Tokenizer<'a> {
    content: &'a [char],
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    #[inline]
    fn starts_with_str(&self, prefix: &str) -> bool {
        if self.content.len() < prefix.len() {
            return false;
        }

        for (index, ch) in prefix.chars().enumerate() {
            if self.content[index] != ch {
                return false;
            }
        }

        true
    }

    pub fn chop(&mut self, n: usize) -> &'a [char] {
        let n = std::cmp::min(n, self.content.len());
        let token = &self.content[..n];
        self.content = &self.content[n..];
        token
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.starts_with_str("http://")
            || self.starts_with_str("https://")
            || self.starts_with_str("www.")
        {
            return Some(
                self.take_while(|c| !c.is_whitespace() && !matches!(c, '<' | '>' | '"' | '\'')),
            );
        }

        let first = self.content[0];
        if first.is_alphabetic() {
            Some(self.take_word_token())
        } else if first.is_numeric() {
            Some(self.take_numeric_token())
        } else {
            self.chop(1);
            self.next_token()
        }
    }

    pub fn take_while<F>(&mut self, mut predicate: F) -> &'a [char]
    where
        F: FnMut(char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn take_word_token(&mut self) -> &'a [char] {
        let mut n = 1;
        while n < self.content.len() {
            let current = self.content[n];
            if current.is_alphanumeric() {
                n += 1;
                continue;
            }

            if is_word_connector(self.content, n) {
                n += 1;
                continue;
            }

            break;
        }

        self.chop(n)
    }

    fn take_numeric_token(&mut self) -> &'a [char] {
        let mut n = 1;
        while n < self.content.len() {
            let current = self.content[n];
            if current.is_numeric() {
                n += 1;
                continue;
            }

            if is_numeric_connector(self.content, n) {
                n += 1;
                continue;
            }

            break;
        }

        self.chop(n)
    }
}

fn is_word_connector(content: &[char], idx: usize) -> bool {
    matches!(content[idx], '-' | '_' | '\'' | '’')
        && idx > 0
        && idx + 1 < content.len()
        && content[idx - 1].is_alphanumeric()
        && content[idx + 1].is_alphanumeric()
}

fn is_numeric_connector(content: &[char], idx: usize) -> bool {
    matches!(content[idx], '.' | ',' | ':' | '/' | '-')
        && idx > 0
        && idx + 1 < content.len()
        && content[idx - 1].is_numeric()
        && content[idx + 1].is_numeric()
}
