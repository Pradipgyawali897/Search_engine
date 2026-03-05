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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_robots_case_insensitive() {
        let content = "User-Agent: *\nDisallow: /tmp\nALLOW: /public\ncrawl-DELAY: 10";
        let robots = parse_robots(content);
        assert!(robots.disallow.contains(&"/tmp".to_string()));
        assert!(robots.allow.contains(&"/public".to_string()));
        assert_eq!(robots.crawl_delay, Some(10));
    }

    #[test]
    fn test_parse_robots_multiple_agents() {
        // These two agents share the same Disallow rule
        let content = "User-agent: MyBot\nUser-agent: *\nDisallow: /private";
        let robots = parse_robots(content);
        assert!(robots.disallow.contains(&"/private".to_string()));
    }

    #[test]
    fn test_parse_robots_multiple_groups() {
        let content = "User-agent: OtherBot\nDisallow: /other\n\nUser-agent: *\nDisallow: /private";
        let robots = parse_robots(content);
        assert!(robots.disallow.contains(&"/private".to_string()));
        assert!(!robots.disallow.contains(&"/other".to_string()));
    }

    #[test]
    fn test_parse_robots_malformed() {
        let content = "User-agent: *\nDisallow\nInvalidLine\nAllow: /ok";
        let robots = parse_robots(content);
        assert!(robots.allow.contains(&"/ok".to_string()));
    }
}