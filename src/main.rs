use clap::Parser;

/* 
    cli and gui(?) application written in rust to keep track of plans and assignments
    ## features
        * cli interface
        * persistant state
        * gtk? when no subcommand or --gui passed
        * interactive
    
    ## cli interface
    * add <task>
    * rm <task>
    * edit <task> 
    * mark <task> --<status>
    
    ### flags
    * --gui 
    * --complete | -c
    * --inprogress | -p
    * --nottouched | -t
    * --help | -h

*/
mod cli;
mod database;
mod unix_time;
mod task;
use verbose_macros::*;
fn main() {
    let args = cli::Args::parse();

    set_verbose(args.verbose);
    set_debug(args.debug);
    verbose!("db: {:?}",database::get_database_path().unwrap());
    debug!("{args:#?}");
}