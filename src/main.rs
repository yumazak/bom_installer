use std::fs;
use std::fs::{File};
use std::path::Path;
use std::env;

fn main() {
    let root;
    let boiler_path;
    let bin_path;
    let exe_path;
    let ignore_path;

    match env::home_dir() {
        Some(p) => {
            root = p.join(".bom");
            boiler_path = root.join("boilerplates");            
            bin_path = root.join("bin");     
            exe_path = root.join("bin/bom.exe");     
            ignore_path = root.join(".bomignore");
        }
        None => panic!("Impossible to get your home dir!"),
    }

    println!("{:?}", &root);
    match fs::read_dir(&root) {
        Ok(dir) => {
            let mut ignore_exists = false;
            for file in dir {
                if &file.unwrap().path().file_name().unwrap().to_str().unwrap().to_string() == ".bomignore" {
                    ignore_exists = true;
                    break;
                }
            }
            if !ignore_exists{
                match File::create(&ignore_path){
                    Ok(_) => println!("create .{:?}", ignore_path),
                    Err(err) => println!("{}", err)
                };
            }
        }
        Err(err) => {
            match fs::create_dir(&root) {
                Ok(_) => {
                    println!("create {:?}", &root);
                    match File::create(&ignore_path){
                        Ok(_) => println!("create .{:?}", ignore_path),
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
        Ok(bin_dir) => {
            println!("already exists {:?}", &bin_path);

            match fs::copy("bom.exe", &exe_path) {
                Ok(_) => println!("create {:?}", &exe_path),
                Err(err) => println!("{}", err)
            };
        }
        Err(err) => {
            match fs::create_dir(&bin_path) {
                Ok(_) => {
                    println!("create {:?}", &bin_path);
                    match fs::copy("bom.exe", &exe_path) {
                        Ok(_) => println!("create {:?}", &exe_path),
                        Err(err) => println!("{}", err)
                    };
                }
                Err(err) => println!("{}", err)
            };
        }
    }
    

 
    println!("finish");
}
