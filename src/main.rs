mod config;
use config::{Config, get_configuration, RmlistConfig, get_rmlist_configuration, create_rmlist};
use std::env;
use std::path;
use std::process;

enum Mode {
    Play,
    Create
}

struct Arguments {
    mode: Mode,
    mlist: String,
}

fn get_args() -> Result<Arguments, String> {
    let user_args:Vec<String> = env::args().collect();
    if user_args.len() < 2 {
        return Err("ERR: Didnt get any argument.".to_string());
    }
    else if user_args.len() < 3 {
        return Err("ERR : Didnt get any rmlist file name.".to_string());
    }
    match &user_args[1][..]{
        "play" => return Ok(Arguments {mode:Mode::Play, mlist:user_args[2].clone()}),
        "create" => return Ok(Arguments {mode:Mode::Create, mlist:user_args[2].clone()}),
        _ => return Err("ERR : Invalid argument.".to_string())
    }
}

fn combine(mlist:&String, path_to_find:&String) -> String{
    let mut p_mlist:String = mlist.clone();
    if !p_mlist.contains(".rmlist") {
        p_mlist.push_str(".rmlist");
    }
    let full_path:String = format!("{}{}", path_to_find, p_mlist);
    match path::Path::new(&full_path).is_file() {
        false => process::exit(1),
        _ => {}
    };
    return full_path;
}

fn play(full_path:&String) {
    let rmlist_content: RmlistConfig = match get_rmlist_configuration(full_path.to_string()) {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };
    for media in &rmlist_content.media {
        match path::Path::new(&media).is_file() {
            false => println!("WARN : `{media}` doesnt exist. Skipping..."),
            _ => {}
        }
    }
    spawn_process("/usr/bin/mpv".to_string(), rmlist_content.media, rmlist_content.other_flag);
}

fn spawn_process(program:String, file:Vec<String>,flag:Vec<String>) {
    process::Command::new(program)
        .args(flag)
        .args(file)
        .spawn()
        .expect("ERR : Failed to spawn the process.");
}

fn main() {
    let prog_args: Arguments = match get_args() {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };
    let user_conf: Config = match get_configuration() {
        Ok(val) => val,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };
    match prog_args.mode {
        Mode::Play => {
            let mlist_path_char: Vec<char> = prog_args.mlist.chars().collect();
            if mlist_path_char[0] == '/' || mlist_path_char[0] == '.'{
                play(&prog_args.mlist);
            } else {
                for i in 0..user_conf.media_list_path.len() {
                    play(&combine(&prog_args.mlist, &user_conf.media_list_path[i]));
                }
            }
        },
        Mode::Create => {
            let mut p_mlist:String = prog_args.mlist.clone();
            if !p_mlist.contains(".rmlist") {
                p_mlist.push_str(".rmlist");
            }
            create_rmlist(&p_mlist);
            println!("WARN : Created the template rmlist file at `{p_mlist}`");
        }
    }
}
