use std::fs;
use std::fs::{File};
use std::path::Path;
use std::env;
use std::process::Command;
use std::io::Write;
fn main() {
    let root;
    let boiler_path;
    let bin_path;
    let exe_path;
    let ignore_path;
    let installer_bin_path;
    let mut ignore_list = vec![".git", ".bomignore"];
    match env::home_dir() {
        Some(p) => {
            root = p.join(".bom");
            boiler_path = root.join("boilerplates");            
            bin_path = root.join("bin");   
            if cfg!(target_os = "windows") {
                exe_path = root.join("bin/bom.exe");
                installer_bin_path = Path::new("bin/bom.exe");
            } else {
                exe_path = root.join("bin/bom");
                installer_bin_path = Path::new("bin/bom");                
            }     
            ignore_path = root.join(".bomignore");
        }
        None => panic!("Impossible to get your home dir!"),
    }

    match fs::read_dir(&root) {
        Ok(dir) => {
            let mut ignore_exists = false;
            for file in dir {
                if &file.unwrap().path().file_name().unwrap().to_str().unwrap().to_string() == ".bomignore" {
                    println!("already exists {:?}", &ignore_path);
                    ignore_exists = true;
                    break;
                }
            }
            if !ignore_exists{
                match File::create(&ignore_path){
                    Ok(mut file) => {
                        for n in &ignore_list {
                            match file.write_all(n.as_bytes()){
                                Err(why) => println!("{:?}", why),
                                Ok(_) => {}
                            }
                            match file.write_all(b"\n"){
                                Err(why) => println!("{:?}", why),
                                Ok(_) => {}
                            }
                        }
                        println!("create {:?}", ignore_path);
                    }
                    Err(err) => println!("{}", err)
                };
            }
        }
        Err(err) => {
            match fs::create_dir(&root) {
                Ok(_) => {
                    println!("create {:?}", &root);
                    match File::create(&ignore_path){
                        Ok(_) => {
                            println!("create .{:?}", ignore_path);
                        }
                        Err(err) => println!("{}", err)
                    };
                }
                Err(err) => {println!("{}", err)}
            };
        }
    }

    match fs::read_dir(&boiler_path) {
        Ok(_) => println!("already exists {:?}", &boiler_path),
        Err(err) => {
            match fs::create_dir(&boiler_path) {
                Ok(_) => {println!("create {:?}", &boiler_path)}
                Err(err) => {println!("{}", err)}
            };
        }
    }
    
    match fs::read_dir(&bin_path) {
        Ok(dir) => {
            println!("already exists {:?}", &bin_path);
            match fs::copy(&installer_bin_path, &exe_path) {
                Ok(_) => println!("create {:?}", &exe_path),
                Err(err) => println!("{}", err)
            };
        }
        Err(err) => {
            match fs::create_dir(&bin_path) {
                Ok(_) => {
                    println!("create {:?}", &bin_path);
                    match fs::copy(&installer_bin_path, &exe_path) {
                        Ok(_) => println!("create {:?}", &exe_path),
                        Err(err) => println!("{}", err)
                    };
                }
                Err(err) => println!("{}", err)
            };
        }
    }
    
    setEnv(&bin_path);
 
    println!("finish");
}

fn setEnv(bin_path: &Path) {

    if cfg!(target_os = "windows") {
        let mut arg = "setx /M path \"%path%;".to_string();
        arg.push_str(bin_path.to_str().unwrap());
        arg.push_str("\"");
        println!("{:?}", arg);
        Command::new("cmd")
                .arg("/C")
                .arg(arg)
                .spawn()                
                .expect("failed to execute process")
    } else {
        let mut arg = "echo \'export PATH=\"$PATH:".to_string();
        arg.push_str(bin_path.to_str().unwrap());
        arg.push_str("\"' >> ~/.bash_profile");
        println!("{}", arg);
        Command::new("sh")
                .arg("-c")
                .arg(arg)
                .spawn()
                .expect("failed to execute process")
    };
}