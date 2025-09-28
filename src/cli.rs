use clap::{Parser};
#[derive(Debug)]
#[derive(Parser)]
pub struct Args {
   #[command(subcommand)]
    command: Commands,
    // flags
    
    // grouping flags
    #[arg(short = 'g', long = "group", global = true)]
    group : bool,
    #[arg(short = 'i', long = "important", global = true)]
    important: bool,

    // status flags
    #[arg(short = 'c', long = "complete")]
    complete: bool,
    #[arg(short = 'p', long = "inprogress")]
    inprogress: bool,
    #[arg(short = 's', long = "notstarted")]
    notstarted: bool,
    #[arg(short = 'o', long = "incomplete")]
    incomplete: bool,

    // misc
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,
    #[arg(long = "debug", global = true)]
    pub debug: bool,
    #[arg(long="gui")]
    pub gui:bool,
    #[arg(long="all")]  
    all:bool,
}
impl Args {

}
#[derive(Debug, clap::Subcommand)]
enum Commands {
    // #[command(short = "a", long = "add")]
    Add {task: String},
    // #[command(short = "rm", long = "remove")]
    Rm {task: String},
    // #[command(short = "ls", long = "list")]
    Ls {task: Option<String>},
    // #[command(short = "e", long = "edit")]
    Edit {task: String},
    // #[command(short = "u", long = "undo")]
    Undo,
}