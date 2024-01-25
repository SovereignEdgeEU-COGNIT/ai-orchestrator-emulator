use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct FileInfo {
    filename: String,
    hash: String,
}

impl FileInfo {

    pub fn get_filename(&self) -> &String {
        return &self.filename;
    }
}

pub struct FileHandler {
    callback: fn(&FileInfo),
    condition: fn(&FileInfo) -> bool
}

impl FileHandler {

    pub fn new(callback: fn(&FileInfo), condition: fn(&FileInfo) -> bool) -> FileHandler {
        FileHandler { callback, condition}
    }
}

pub struct Cache {
    local_hashes: HashSet<FileInfo>,
    listeners: Vec<FileHandler>
}

impl Cache {

    pub fn new() -> Cache {
        Cache{local_hashes: HashSet::new(), listeners: Vec::new()}
    }

    pub fn register_listener(&mut self, handler: FileHandler) {
        self.listeners.push(handler);
    }

    pub fn in_cache (&self, file_info: &FileInfo) -> bool {
        return self.local_hashes.contains(file_info);
    }

    pub fn process_file(&mut self, file_info: FileInfo) {
    
        self.listeners
            .iter()
            .filter(|file_handler| (file_handler.condition)(&file_info))
            .for_each(|file_handler| (file_handler.callback)(&file_info));

            self.local_hashes.insert(file_info);
    }
}




