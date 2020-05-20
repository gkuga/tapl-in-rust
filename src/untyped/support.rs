#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Info {
    FileInfo(String, usize, usize),
    Unknown,
}

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub struct WithInfo {
    pub i: Info,
    pub v: String,
}
