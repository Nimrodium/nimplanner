#[derive(PartialEq,Eq)]
enum Status{
    NotTouched,
    InProgress,
    Complete,
}

/// status on how long until a task is due
enum DueStatus{
    Upcoming(usize),
    Tomorrow,
    Today,
    Late(usize)
}

impl DueStatus{

}

pub struct NoteEntry{
    title:String,
    body:String,
    date:u64,
}

pub struct Task {
    name:String,
    details:String,
    status:Status,
    notes:Vec<NoteEntry>,
    
    date_entered:u64, // unix timestamp likely maybe ill make a wrapper class or just import a crate for it
    date_due:u64,
    due_status:DueStatus,
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
            due_status: DueStatus::Upcoming(1)
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
    pub fn get_due_status(&self) -> DueStatus{
        
    }
}