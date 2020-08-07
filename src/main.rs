#[macro_use]
extern crate clap;
extern crate xdg;
use clap::App;

fn main() -> Result<(), String> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("list") => {
            println!("list subcommmand found")
            // Call list_configs return array out to std_out line by line
        }
        Some("delete") => {
            println!("delete subcommmand found")
            // Write Logic to handle Args
            // pass arg to delete_config function
        }
        Some("switch") => {
            println!("switch subcommmand found")
            // Write logic to handle Args
            // Pass 'file' arg to switch_config function
        }
        Some("import") => {
            println!("import subcommmand found")
            // Write logic to handle import commands
            // Pass 'name' arg to import_config function
        }
        Some("show") => {
            println!("show subcommmand found")
            // Write logic to handle show commands.
            // call show_command function with arg
        }
        Some("context") => {
            println!("context subcommmand found")
            // Write Logic to Handle context arguements
        }
        Some(&_) => println!("No valid subcommand found"),
        None => println!("No subcommand found"),
    }
}
