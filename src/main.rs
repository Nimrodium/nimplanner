mod task;
mod cli;
fn get_database_path() -> String {
    if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").unwrap();
        format!("{appdata}\\nimplanner\\db\\")
    } else {
        let home = std::env::var("HOME").unwrap();
        format!("{home}/.local/share/nimplanner/db/")
    }
}

use clap::Parser;
fn main() {

    let args = cli::Args::parse();
    println!("{args:#?}");
    println!("{}", get_database_path());

}

