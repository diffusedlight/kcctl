#[macro_use]
extern crate clap;
extern crate config;
extern crate dirs; 

use clap::App;
use std::fs;
use std::env;
use std::path::PathBuf;


// Modules
// mod config;

fn main() -> () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut config_dir_path = dirs::config_dir().unwrap();
    config_dir_path.push("kcctl/configs");

    let mut base_config_path = dirs::config_dir().unwrap();
    base_config_path.push("kcctl");

    if base_config_path.exists() == false {
        fs::create_dir(&base_config_path);
        fs::create_dir(&config_dir_path);
    } else if config_dir_path.exists() == false {
        fs::create_dir(&config_dir_path);
    };

    let mut kubeconfig_path = match env::var("KUBECONFIG") {
        Ok(v) => PathBuf::from(v),
        Err(_e) => {
            let mut temp_config_path = dirs::home_dir().unwrap();
            temp_config_path.push(".kube/config");
            temp_config_path
        },
    };
    

    match matches.subcommand_name() {
        Some("list") => {
            println!("list subcommmand found");
            // Call list_configs return array out to std_out line by line
        }
        Some("delete") => {
            println!("delete subcommmand found");
            // Write Logic to handle Args
            // pass arg to delete_config function
        }
        Some("switch") => {
            println!("switch subcommmand found");
            // Write logic to handle Args
            // Pass 'file' arg to switch_config function
        }
        Some("import") => {
            println!("import subcommmand found");
            // Write logic to handle import commands
            // Pass 'name' arg to import_config function
        }
        Some("show") => {
            println!("show subcommmand found");
            // Write logic to handle show commands.
            // call show_command function with arg
        }
        Some("context") => {
            println!("context subcommmand found");
            // Write Logic to Handle context arguements
        }
        Some("configure") => {
            println!("configure subcommand found");
            // Ask for some input: Config Dir path, Kubeconfig location.
            // Write the inputs to a config file (toml)
        }
        Some(&_) => println!("No valid subcommand found"),
        None => println!("No subcommand found"),
    }
}