use std::net::ToSocketAddrs;

pub fn resolve_ip_to_dns(host: &str) -> Result<String, Box<dyn std::error::Error>> {
    let host = format!("{}:80", host);
    match host.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                Ok(addr.to_string())
            } else {
                Err("No IP found".into())
            }
        }
        Err(e) => Err(e.into()),
    }
}
