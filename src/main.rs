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
mod task;
fn main() {
    println!("Hello, world!");
}