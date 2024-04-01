use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub directives: String,
    pub file: LogFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogFile {
    pub enabled: bool,
    pub path: String,
    pub directives: String,
}
