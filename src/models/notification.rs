use std::time::{self, SystemTime};

#[derive(PartialEq, Clone, Debug)]
pub struct Notification {
    id: SystemTime,
    title: String,
    description: Option<String>,
    permanent: bool,
    timeout: u32,
}

impl Notification {
    pub fn new(
        title: String, 
        description: Option<String>,
        permanent: Option<bool>,
        timeout: Option<u32>
    ) -> Notification {
        Notification {
            id: time::SystemTime::now(),
            title,
            description,
            permanent: permanent.unwrap_or(false),
            timeout: timeout.unwrap_or(2000)
        }
    }
    pub fn id(&self) -> SystemTime {
        self.id
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn permanent(&self) -> &bool {
        &self.permanent
    }
    pub fn timeout(&self) -> &u32 {
        &self.timeout
    }
}