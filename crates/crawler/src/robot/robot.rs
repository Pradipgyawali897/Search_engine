use super::server::check_robot;
use super::utils::Robots;
use super::utils::parse_robots;

pub async fn get_robot_content(domain: &str) -> Option<Robots> {
    let is_robot = check_robot(domain).await;
    match is_robot {
        Ok(Some(content)) => {
            let robots = parse_robots(&content);
            println!("The site contains the robot.txt");
            println!("Robots.txt: {:#?}", robots);
            Some(robots)
        }
        Ok(None) => None,
        Err(e) => {
            println!("Error: {:#?}", e);
            None
        }
    }
}
