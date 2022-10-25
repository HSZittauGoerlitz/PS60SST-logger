use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PS60SSTconfig {
    #[serde(rename = "Com port")]
    pub com_port: String,
    #[serde(rename = "File location")]
    pub f_loc: String,
    #[serde(rename = "File name")]
    pub f_name: String
}

impl PS60SSTconfig {
    pub fn new() -> PS60SSTconfig {
        PS60SSTconfig {
            com_port: "COM1".to_owned(),
            f_loc: "D:/".to_owned(),
            f_name: "weight_log.txt".to_owned()
        }
    }

    pub fn load_json(config_file: &str)
    -> PS60SSTconfig
    {
        let cfg: PS60SSTconfig;
        match std::fs::read_to_string(config_file) {
            Err(_) => {
                println!("Error open config file: {:?}", config_file);
                println!("Trying with default configuration.");
                cfg = PS60SSTconfig::new();
            },
            Ok(data) => {
                match serde_json::from_str(&data) {
                    Ok(res) => {
                        cfg = res;
                    },
                    Err(e) => {
                        println!("Error reading config: {:?}", e);
                        println!("Trying with default configuration.");
                        cfg = PS60SSTconfig::new();
            }
        }}};
        cfg
    }
}
