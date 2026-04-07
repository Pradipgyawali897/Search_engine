#[derive(Debug)]
pub struct Robots {
    pub disallow: Vec<String>,
    pub allow: Vec<String>,
    pub crawl_delay: Option<u64>,
}

pub fn parse_robots(content: &str) -> Robots {
    let mut robots = Robots {
        disallow: Vec::new(),
        allow: Vec::new(),
        crawl_delay: None,
    };

    let mut current_group_matches = false;
    let mut in_directive_block = false;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() < 2 {
            continue;
        }

        let key = parts[0].trim().to_lowercase();
        let value = parts[1].trim();

        match key.as_str() {
            "user-agent" => {
                if in_directive_block {
                    current_group_matches = false;
                    in_directive_block = false;
                }

                if value == "*" {
                    current_group_matches = true;
                }
            }
            "disallow" => {
                in_directive_block = true;
                if current_group_matches && !value.is_empty() {
                    robots.disallow.push(value.to_string());
                }
            }
            "allow" => {
                in_directive_block = true;
                if current_group_matches && !value.is_empty() {
                    robots.allow.push(value.to_string());
                }
            }
            "crawl-delay" => {
                in_directive_block = true;
                if current_group_matches {
                    if let Ok(delay) = value.parse::<u64>() {
                        robots.crawl_delay = Some(delay);
                    }
                }
            }
            _ => {}
        }
    }

    robots
}
