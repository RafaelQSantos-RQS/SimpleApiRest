use std::net::SocketAddr;
use tracing_subscriber::fmt::format::FmtSpan;

pub enum LogFormat {
    Pretty,
    Json
}

pub struct Config {
    pub server_addr: SocketAddr,
    pub log_format: LogFormat,
}

impl Config {
    pub fn from_env() -> Self {
        let port: u16 = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let log_format = match std::env::var("LOG_FORMAT").as_deref() {
            Ok("json") => LogFormat::Json,
            _ => LogFormat::Pretty,
        };

        Self {
            server_addr: SocketAddr::from(([0,0,0,0], port)),
            log_format
        }
    }

    pub fn init_logging(&self) {
        let subscriber = tracing_subscriber::fmt()
            .with_span_events(FmtSpan::CLOSE);

        match self.log_format {
            LogFormat::Json => subscriber.json().init(),
            LogFormat::Pretty => subscriber.init(),
        }
    }
}