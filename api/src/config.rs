use std::net::SocketAddr;

/// Server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Server bind address
    pub addr: SocketAddr,
    /// API title for OpenAPI docs
    pub title: String,
    /// API description
    pub description: String,
    /// API version
    pub version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:8001".parse().unwrap(),
            title: "DeepPDCFR Solver API".to_string(),
            description: "REST API for querying Nash-equilibrium strategies in No-Limit Hold'em. Uses PioSOLVER syntax for bet sizes and hand ranges.".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}

impl Config {
    /// Create a new config with custom port
    pub fn with_port(port: u16) -> Self {
        let mut config = Self::default();
        config.addr = format!("0.0.0.0:{}", port).parse().unwrap();
        config
    }
}
