pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            id: 0,
            name: name,
            description: description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

pub struct Story {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            id: 0,
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    pub last_item_id: i32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}
