use search_engine::crawler::{dns_resolver::dns_resolver, seed_manager::consume_seed};

fn main() {
    let seed = consume_seed::consume_seeds_from_file();
    println!("{:?}", seed);
    let seed_manager = consume_seed::create_seed();
    println!("{:?}", seed_manager);
    for ip in seed_manager.iter() {
        let address = ip.host().unwrap().to_string();
        let dns_record = dns_resolver::resolve_ip_to_dns(&address);
        println!("{:?}", dns_record);
    }
}
