/* 
    cli and gui(?) application written in rust to keep track of plans and assignments
    ## features
        * cli interface
        * persistant state
        * gtk? when no subcommand or --gui passed
        * interactive
    
    ## cli interface
    * add
    * rm
    * edit
    * 

*/
fn main() {
    println!("Hello, world!");
}

enum Status{
    NotTouched,
    InProgress
    Complete,

}

pub struct NoteEntry{
    title:String,
    body:String,
    date:u64,
}

pub struct Task {
    name:String,
    details:String,
    date_entered:u64, // unix timestamp likely maybe ill make a wrapper class or just import a crate for it
    date_due:u64,
    status:Status
    notes:Vec<NoteEntry>,
}
