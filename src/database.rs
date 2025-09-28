use std::{env, fs::File, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use serde_json;
use verbose_macros::*;

use crate::task::Task;
/*  
    database.rs manages loading the database json.
    contains database class
*/

pub struct DbManager{
    database_dir:PathBuf,
    database:Database,
}

impl DbManager {
    pub fn new() -> Result<Self, String> {
        let database_dir = get_database_path()?;
        let database  = Database::load(&database_dir.join("db.json"))?;
        Ok(Self { database_dir , database})
    }
}

#[derive(Serialize,Deserialize)]
struct Database{
    tasks:Vec<Task>,
    // meta ?

}
impl Database{
    fn load(json:&Path) -> Result<Self, String>{
        verbose!("loading database: {json:?}");
        let file = File::open(json).map_err(|e|format!("failed to open {json:?}: {e}"))?;
        serde_json::from_reader(file).map_err(|e|format!("failed to deserialize {json:?} {e}"))?
    }
    fn write(target:&Path) -> Result<(),String> {
        todo!()
    }
}

fn get_var(var:&str) -> Result<String,String>{
    let err = if cfg!(windows){
        format!("%{var}%")
    }else{format!("${var}")};
    env::var(var).map_err(|e|format!("Environment Variable {err} Not Set ({e})"))
}
/// retrieves database path
/// * Windows
/// > `%APPDATA%\nimplanner\`
/// * MacOS
/// > `$HOME/Library/Application Support/nimplanner/`
/// * Linux
/// > `$XDG_DATA_HOME/nimplanner/`
pub fn get_database_path() -> Result<PathBuf,String>{
    
    let data_home = if cfg!(windows){
        let appdata = get_var("APPDATA")?;
        PathBuf::from(appdata)
    }else if cfg!(target_os = "macos"){
        let home = get_var("HOME")?;
        PathBuf::from(home).join("Library/Application Support/")
    }else{
        match env::var("XDG_DATA_HOME"){
            Ok(data) => {
                PathBuf::from(data)
            },
            Err(e) => {
                let home = get_var("HOME")?;
                let fallback = PathBuf::from(home).join(".local/share");
                println!("{e}\nfalling back to {fallback:?}");
                fallback
            },
        }
    };
    Ok(data_home.join("nimplanner"))
}