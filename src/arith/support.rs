pub mod error {
    #[allow(dead_code)]
    #[derive(PartialEq, Clone, Hash, Eq, Debug)]
    pub enum Info {
        FileInfo(String, String, usize),
        Unknown,
    }
}
