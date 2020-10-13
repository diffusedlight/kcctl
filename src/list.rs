use std::fs;
use std::io;
use std::path;

pub fn list_configs(config_dir_path: path::PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(config_dir_path)? {
        let dir = entry?;
        let file_name = dir.file_name().into_string().unwrap();
        println!("{}", file_name);
    }
    Ok(())
}
