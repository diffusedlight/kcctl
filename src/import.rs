use std::fs;
use std::path;

pub fn import_config(config_dir_path: path::PathBuf, kubeconfig_path: path::PathBuf, config_name: String) {
  let mut config_path = config_dir_path;
  config_path.push(&config_name);
  match fs::copy(kubeconfig_path, config_path) {
    Ok(t) => println!("Import Success"),
    Err(e) => eprintln!("Import Failed: {}", e)
  }
}
