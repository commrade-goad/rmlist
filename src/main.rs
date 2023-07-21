mod config;
use config::{create_rmlist, get_configuration, get_rmlist_configuration, Config, RmlistConfig};
use std::env;
use std::path;
use std::process;
use std::fs;

enum Mode {
    Play,
    Create,
}

struct Arguments {
    mode: Mode,
    mlist: String,
}

const MEDIA_PLAYER: &str = "/usr/bin/mpv";

fn get_args() -> Result<Arguments, String> {
    let user_args: Vec<String> = env::args().collect();
    if user_args.len() < 2 {
        return Err("ERR: Didnt get any argument.".to_string());
    } else if user_args.len() < 3 {
        return Err("ERR : Didnt get any rmlist file name.".to_string());
    }
    match &user_args[1][..] {
        "play" | "p" => {
            return Ok(Arguments {
                mode: Mode::Play,
                mlist: user_args[2].clone(),
            })
        }
        "create" | "c" => {
            return Ok(Arguments {
                mode: Mode::Create,
                mlist: user_args[2].clone(),
            })
        }
        _ => return Err("ERR : Invalid argument.".to_string()),
    }
}

fn combine(mlist: &String, path_to_find: &String) -> Result<String, String> {
    let mut p_mlist: String = mlist.clone();
    if !p_mlist.contains(".rmlist") {
        p_mlist.push_str(".rmlist");
    }
    let full_path: String = format!("{}{}", path_to_find, p_mlist);
    match path::Path::new(&full_path).is_file() {
        false => {
            return Err(format!(
                "ERR : the specified file `{full_path}` doesnt exist."
            ))
        }
        _ => {}
    };
    return Ok(full_path);
}

fn play(full_path: &String) {
    let mut rmlist_content: RmlistConfig = match get_rmlist_configuration(full_path) {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    for index in 0..rmlist_content.media.len() {
        if !&rmlist_content.media[index].contains("https://") {
            match path::Path::new(&rmlist_content.media[index]).is_file() {
                false => println!("WARN : `{}` doesnt exist. Skipping...", rmlist_content.media[index]),
                _ => {}
            }
        } else if path::Path::new(&rmlist_content.media[index]).is_dir() {
            match fs::read_dir(&rmlist_content.media[index]) {
                Ok(val) => {
                    let tmp_vec: fs::ReadDir = val;
                    for file in tmp_vec {
                        rmlist_content.media.push(file.unwrap().path().display().to_string());
                    }
                },
                Err(_) => {
                    println!("WARN : `{}` folder doesnt exist. Skipping...", rmlist_content.media[index]);
                },
            }
            rmlist_content.media.remove(index);
        }
    }
    spawn_process(
        MEDIA_PLAYER,
        rmlist_content.media,
        rmlist_content.other_flag,
    );
}

fn spawn_process(program: &str, file: Vec<String>, flag: Vec<String>) {
    process::Command::new(program)
        .args(flag)
        .args(file)
        .spawn()
        .expect("ERR : Failed to spawn the process.")
        .wait()
        .expect("WARN : Failed to wait the mpv process");
}

fn print_help() {
    println!("USAGE :");
    println!("* Play rmlist:");
    println!("    rmlist play file  or  rmlist p file");
    println!("    rmlist play /path/to/file.rmlist  or  rmlist p /path/to/file.rmlist");
    println!("* Create rmlist file:");
    println!("    rmlist create /path/to/file  or  rmlist c /path/to/file");
}

fn main() {
    let prog_args: Arguments = match get_args() {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            print_help();
            process::exit(1);
        }
    };
    let user_conf: Config = match get_configuration() {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            print_help();
            process::exit(1);
        }
    };
    match prog_args.mode {
        Mode::Play => {
            let mlist_path_char: Vec<char> = prog_args.mlist.chars().collect();
            if mlist_path_char[0] == '/' || mlist_path_char[0] == '.' {
                play(&prog_args.mlist);
                return;
            }
            let mut empty_path: Vec<String> = Vec::new();
            for i in 0..user_conf.media_list_path.len() {
                match combine(&prog_args.mlist, &user_conf.media_list_path[i]) {
                    Ok(val) => play(&val),
                    Err(err) => {
                        empty_path.push(err);
                        if i == user_conf.media_list_path.len() - 1 {
                            empty_path.iter().for_each(|val| println!("{val}"));
                            process::exit(1);
                        }
                    }
                }
            }
        }
        Mode::Create => {
            let mut p_mlist: String = prog_args.mlist.clone();
            if !p_mlist.contains(".rmlist") {
                p_mlist.push_str(".rmlist");
            }
            match create_rmlist(&p_mlist) {
                Ok(()) => {}
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
            println!("WARN : Created the template rmlist file at `{p_mlist}`");
        }
    }
}
