pub struct ProcessInfo {
    pub name: String,
    pub cpu: f32,
    pub ram_mb: u64,
}

pub struct GroupInfo {
    pub name: String,
    pub count: usize,
    pub cpu: f32,
    pub ram_mb: u64,
}