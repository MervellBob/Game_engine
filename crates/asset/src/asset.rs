use std::fs::File;

///
/// All common methods between assets
///
pub trait Asset {
    fn new(file: File) -> Self;
    /// It should be `Vec<u8>` in "contents_raw" field
    fn read_raw(&mut self);
}
