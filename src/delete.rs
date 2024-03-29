use std::fs;
use std::path;

pub fn delete_specified_config(config_dir_path: path::PathBuf, name: String) {
    // Construct Config File path
    let mut config_file = config_dir_path; 
    config_file.push(&name.trim().to_lowercase());

    // Attempt to get metadata from file
    match fs::metadata(&config_file) {
        Ok(_t) => (),
        Err(_e) => return println!("File not found.")
    }

    // Attempt to remove file specified
    match fs::remove_file(config_file) {
        Ok(_t) => println!("{} config removed", name),
        Err(_e) => eprintln!("Could not delete {}", name)
    }
}

// Removes file located at kubeconfig_path
pub fn delete_current_config(kubeconfig_path: path::PathBuf) {
    match fs::remove_file(kubeconfig_path) {
        Ok(_t) => println!("Active config removed"),
        Err(e) => eprintln!("Could not delete {}", e)
    }
}