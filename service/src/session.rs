
#[derive(Clone)]
pub struct Session {
    pub seq_num: u32,
}

impl Session {
    pub fn new() -> Self {
        Self {
            seq_num: 0,
        }
    }
}
