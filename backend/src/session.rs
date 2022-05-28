pub struct Session {
    users: [u32; 2],
}

impl Session {
    pub fn new(users: [u32; 2]) -> Self {
        Self {
            users,
        }
    }
}