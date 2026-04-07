use spyder::robot::utils::parse_robots;

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
