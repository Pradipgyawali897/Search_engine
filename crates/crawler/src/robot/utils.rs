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

    let mut applies_to_us = false;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if line.starts_with("User-agent:") {
            let agent = line.split(':').nth(1).unwrap().trim();
            applies_to_us = agent == "*";
        }

        if applies_to_us {
            if line.starts_with("Disallow:") {
                let path = line.split(':').nth(1).unwrap().trim();
                robots.disallow.push(path.to_string());
            }

            if line.starts_with("Allow:") {
                let path = line.split(':').nth(1).unwrap().trim();
                robots.allow.push(path.to_string());
            }

            if line.starts_with("Crawl-delay:") {
                let delay = line.split(':').nth(1).unwrap().trim();
                robots.crawl_delay = delay.parse().ok();
            }
        }
    }

    robots
}