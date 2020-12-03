use std::fs;
use std::path;

pub fn import_config(config_dir_path: path::PathBuf, kubeconfig_path: path::PathBuf, config_name: String) {
  let name = String::from(config_name); 
  let mut config_path = config_dir_path;
  config_path.push(&name.trim().to_lowercase());
  match fs::copy(kubeconfig_path, config_path) {
    Ok(_t) => println!("Import Success"),
    Err(e) => eprintln!("Import Failed: {}", e)
  }
}
