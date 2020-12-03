use std::fs;
use std::path;

pub fn switch_config(config_dir_path: path::PathBuf, kubeconfig_path: path::PathBuf, config_name: String) {
    let name = String::from(config_name);
    let mut config_path = config_dir_path;
    config_path.push(&name.trim().to_lowercase());
    match fs::copy(config_path, kubeconfig_path) {
        Ok(_t) => println!("Switched config to {}", name),
        Err(e) => eprintln!("Couldn't switch config")
    }
}
