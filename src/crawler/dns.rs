use std::net::ToSocketAddrs;

pub fn resolve_ip_to_dns(host: &str) -> Result<String, Box<dyn std::error::Error>> {
    let host = format!("{}:80", host);
    match host.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                return Ok(addr.to_string());
            } else {
                return Err("No IP found".into());
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_localhost() {
        let result = resolve_ip_to_dns("localhost");
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_invalid_host_returns_error() {
        let result = resolve_ip_to_dns("this.host.does.not.exist.invalid");
        assert!(result.is_err());
    }
}
