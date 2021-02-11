#[macro_use]
extern crate clap;
extern crate config;
extern crate dirs;

pub mod list;
pub mod import;
pub mod delete;
pub mod switch;

use clap::App;
use list::list_configs;
use import::import_config;
use delete::delete_current_config;
use delete::delete_specified_config;
use switch::switch_config;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut config_dir_path = dirs::config_dir().unwrap();
    println!("{:?}", config_dir_path);
    config_dir_path.push("kcctl/configs");

    let mut base_config_path = dirs::config_dir().unwrap();
    base_config_path.push("kcctl");
    fs::create_dir_all(&base_config_path);
    fs::create_dir_all(&config_dir_path);

    let kubeconfig_path = match env::var("KUBECONFIG") {
        Ok(v) => PathBuf::from(v),
        Err(_e) => {
            let mut temp_config_path = dirs::home_dir().unwrap();
            temp_config_path.push(".kube");
            temp_config_path.push("config");
            temp_config_path
        }
    };

    match matches.subcommand() {
        ("list", Some(_list_matches)) => {
            list_configs(config_dir_path);
        }

        ("delete", Some(delete_matches)) => {
            println!("delete subcommmand found");
            if delete_matches.is_present("current") {
                println!("current flag found");
                delete_current_config(kubeconfig_path);
            } else if let Some(file) = delete_matches.value_of("file") {
                println!("Value of file is: {:?}", file); 
                delete_specified_config(config_dir_path, file.to_string());
            }
        }

        ("switch", Some(switch_matches)) => {
            println!("switch subcommmand found");
            if let file = switch_matches.value_of("file").unwrap() {
                println!("Value of file is: {:?}", file); 
                switch_config(config_dir_path, kubeconfig_path, file.to_string());
            }


        }
        ("import", Some(import_matches)) => {
            println!("import subcommmand found");
            if let name = import_matches.value_of("name").unwrap() {
                import_config(config_dir_path, kubeconfig_path, name.to_string());
            }
        }

        ("show", Some(show_matches)) => {
            println!("show subcommmand found");
            if show_matches.is_present("current") {
                println!("current flag found")
            } else if let Some(config) = show_matches.value_of("config") {
                println!("Value of config is {:?}", config)
            }

        }

        ("context", Some(context_matches)) => {
            println!("context subcommmand found");
            if context_matches.is_present("list") {
                println!("List subcommand found")
            } else if context_matches.is_present("switch") {
                println!("Switch subcommand found");
                if let Some(switch_matches) = context_matches.subcommand_matches("switch") {
                    let input = switch_matches.value_of("INPUT").unwrap(); // Can safely call unwrap() as arg is required by clap
                    println!("The value of input is: {:?}", input);
                }
            }   
        }

        _ => println!("No subcommand found"),
    }
}
