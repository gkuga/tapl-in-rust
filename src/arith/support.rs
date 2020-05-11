#[allow(dead_code)]
#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Info {
    FileInfo(String, usize, usize),
    Unknown,
}
