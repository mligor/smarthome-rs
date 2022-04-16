pub struct Event {
    pub name: String,
}

impl Event {
    pub fn new(name: String) -> Event {
        Event { name }
    }
}
