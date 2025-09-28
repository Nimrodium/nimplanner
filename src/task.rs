use serde::{Deserialize, Serialize};

use crate::unix_time::UnixStamp;
#[derive(PartialEq,Eq,Serialize,Deserialize,Debug)]
#[repr(u8)]
pub enum Status{
    NotTouched,
    InProgress,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteEntry{
    title:String,
    body:String,
    date:UnixStamp,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Task {
    name:String,
    details:String,
    status:Status,
    notes:Vec<NoteEntry>,
    date_entered:UnixStamp, // unix timestamp likely maybe ill make a wrapper class or just import a crate for it
    date_due:UnixStamp,
}
impl Task{
    pub fn new() -> Self {
        Self {
            name: todo!(),
            details: todo!(),
            status: todo!(),
            notes: todo!(),
            date_entered: todo!(),
            date_due: todo!(),
        }
    }
    /// returns true if status changed, false if new status and old status are the same
    pub fn set_status(&mut self,status:Status) -> bool {
        let same = self.status == status;
        self.status = status;
        !same
    }
    pub fn set_due(&mut self,due_stamp:u64){
        todo!()
    }
}