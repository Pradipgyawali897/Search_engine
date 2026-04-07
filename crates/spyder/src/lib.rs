pub mod dns;
pub mod frontier;
pub mod parser;
pub mod robot;
pub mod url_handler;

pub use dns::resolver::resolve_ip_to_dns;
pub use frontier::loader::{consume_seeds_from_file, create_seed};
pub use frontier::manager::Frontier;
pub use robot::robot::get_robot_content;
pub use robot::utils::Robots;
pub use url_handler::url_normalizer::normalize_url;
