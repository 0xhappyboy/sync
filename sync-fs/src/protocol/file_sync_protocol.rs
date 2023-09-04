use crate::file::file::File;

#[derive(Debug, Clone)]
pub struct FileSyncProtocol {
    pub name: [u8; 100],
    pub length: u64,
    pub data: Vec<u8>,
}

impl FileSyncProtocol {
    pub fn New(file: File) -> FileSyncProtocol {
        /// file name
        let mut f_name: [u8; 100] = [0u8; 100];
        f_name[..file.name.as_bytes().len()].copy_from_slice(file.name.as_bytes());
        FileSyncProtocol {
            name: f_name,
            length: 10,
            data: [0u8; 100].to_vec(),
        }
    }
}
