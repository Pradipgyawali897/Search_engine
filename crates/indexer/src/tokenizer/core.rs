use super::utils::{is_valid_url, save_url};

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

        let mut i = 0;
        for c in prefix.chars() {
            if self.content[i] != c {
                return false;
            }
            i += 1;
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
            let url_chars = self.take_while(|c| {
                !c.is_whitespace() && c != '<' && c != '>' && c != '"' && c != '\''
            });
            let url_str: String = url_chars.iter().collect();

            if is_valid_url(&url_str) {
                let category = super::link_filter::classify_link(&url_str);
                save_url(&url_str, category);
            }
            return Some(url_chars);
        }

        let first = self.content[0];
        if first.is_alphabetic() {
            Some(self.take_while(|c| c.is_alphanumeric()))
        } else if first.is_numeric() {
            Some(self.take_while(|c| c.is_numeric()))
        } else {
            Some(self.chop(1))
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
}
