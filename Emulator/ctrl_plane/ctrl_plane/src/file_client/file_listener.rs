use crate::file_client::FileInfo;

pub struct FileListener {
    callback: fn(&FileInfo),
    condition: fn(&FileInfo) -> bool
}

impl FileListener {

    pub fn new(callback: fn(&FileInfo), condition: fn(&FileInfo) -> bool) -> FileListener {
        FileListener { callback, condition}
    }

    pub fn callback(&self, file_info: &FileInfo) {
        (self.callback)(file_info);
    }

    pub fn condition(&self, file_info: &FileInfo) -> bool {
        (self.condition)(file_info)
    }
}