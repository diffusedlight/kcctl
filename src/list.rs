use std::fs;
use std::io;
use std::path;

pub fn list_configs(config_dir_path: path::PathBuf) -> io::Result<()> {
    // Iterates over the entries within the directory
    for entry in fs::read_dir(config_dir_path)? {
        let dir = entry?;
        // grabs file_name from metadata
        let file_name = dir.file_name().into_string().unwrap();
        println!("{}", file_name);
    }
    Ok(())
}
