use url::Url;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Tokenizer<'a> {
    content: &'a [char],
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn starts_with_str(&self, prefix: &str) -> bool {
        let prefix_chars: Vec<char> = prefix.chars().collect();
        if self.content.len() < prefix_chars.len() {
            return false;
        }
        &self.content[..prefix_chars.len()] == &prefix_chars[..]
    }

    fn save_url(url: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("discovered_urls.txt")
        {
            let _ = writeln!(file, "{}", url);
        }
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

        if self.starts_with_str("http://") || self.starts_with_str("https://") || self.starts_with_str("www.") {
            let url_chars = self.take_while(|c| !c.is_whitespace() && c != '<' && c != '>' && c != '"' && c != '\'');
            let url_str: String = url_chars.iter().collect();
            
            let to_validate = if url_str.starts_with("www.") {
                format!("https://{}", url_str)
            } else {
                url_str.clone()
            };

            if Url::parse(&to_validate).is_ok() {
                Self::save_url(&url_str);
                return Some(url_chars);
            } else {
                return Some(url_chars);
            }
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
