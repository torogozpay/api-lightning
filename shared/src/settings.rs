use lazy_static::lazy_static;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub macaroon_file: String,    
    pub cert_file: String,    
    pub host: String,
    pub port: u32,

    //duración de invoice
    pub expiry: i64, 
    //hold invoice cltv delta (expiration time in blocks)
    pub cltv_expiry: u64,
    //maxima cantidad de saltos que queremos que el nodo de para intentar efectuar el pago
    pub max_paths: i32,  
    //cantidad de tiempo para tratar de encontrar una ruta
    pub pathfinding_timeout: i32,  
    //la cantidad maxima de fee que esta mos dispuestos a pagar por el ruteo
    pub max_fee: f64,  
    //el ID del canal del peer por el cual queremos sacar el pago de nuestro nodo
    pub out: String, 
}    



#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub database_url: String,    
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Api {
    pub api_key: String,
    pub api_username: String,
    pub api_password: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub node: Node,
    pub server: Server,
    pub log: Log,
    pub api: Api,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./shared/src/config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./shared/src/config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;
        
        s.try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

lazy_static! {
    pub static ref CONFIG : Settings = Settings::new().expect("Config can be loaded");
}