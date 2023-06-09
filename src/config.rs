use serde_derive::Deserialize;
use toml;
use std::env;
use std::path;
use std::fs;
use std::io::Write;

#[derive(Deserialize)]
pub struct RmlistConfig {
    pub media: Vec<String>,
    pub other_flag: Vec<String>
}

#[derive(Deserialize)]
pub struct Config {
    pub media_list_path: Vec<String>,
}

pub fn get_configuration() -> Result<Config, String>{
    let mut path_to_conf: String = match env::var("HOME") {
        Ok(val) => val,
        Err(err) => return Err(err.to_string())
    };
    path_to_conf.push_str("/.config/rmlist.toml");
    match path::Path::new(&path_to_conf).is_file() {
        false => {
            let mut create_config: fs::File = fs::File::create(&path_to_conf).expect("Error encountered while creating file!");
            match create_config.write_all(b"media_list_path = []") {
                Ok(_) => {},
                Err(err) => return Err(err.to_string())
            };
            println!("WARN : Created the config file.");
            return Err("WARN : Exiting..".to_string());
        }
        true => {
            let contents: String = match fs::read_to_string(path_to_conf) {
                Ok(val) => val,
                Err(err) => return  Err(err.to_string())
            };
            let mut data: Config = match toml::from_str(&contents) {
                Ok(d) => d,
                Err(_) => {
                    return Err("WARN : Failed to parse the config file!".to_string());
                }
            };
            for i in 0..data.media_list_path.len() {
                let path_char: Vec<char> = data.media_list_path[i].chars().collect();
                match path_char[path_char.len()-1]{
                    '/' => {},
                    _ => data.media_list_path[i].push_str("/")
                }
            }
            return Ok(data);
        }
    }
}

pub fn get_rmlist_configuration(path_to_rmlist:String) -> Result<RmlistConfig, String>{
    match path::Path::new(&path_to_rmlist).is_file() {
        true => {
            let contents: String = match fs::read_to_string(path_to_rmlist) {
                Ok(val) => val,
                Err(err) => return  Err(err.to_string())
            };
            let data = match toml::from_str(&contents) {
                Ok(d) => d,
                Err(_) => {
                    return Err("ERR : Failed to parse rmlist file!".to_string());
                }
            };
            return Ok(data);
        }
        _ => return Err("ERR : The specified path for rmlist file doesnt exist.".to_string())
    }
}

pub fn create_rmlist(path_to_rmlist:&String) -> u8{
    let mut create_config: fs::File = fs::File::create(&path_to_rmlist).expect("WARN : Error encountered while creating file!");
    match create_config.write_all(b"media = []\nother_flag = []") {
        Ok(_) => {},
        Err(_) => return 1
    }
    return 0;
}

