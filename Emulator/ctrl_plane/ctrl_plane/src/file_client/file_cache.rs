use std::collections::HashSet;

use crate::file_client::{FileInfo, file_listener::FileListener};


pub struct Cache {
    local_hashes: HashSet<FileInfo>,
    listeners: Vec<FileListener>
}

impl Cache {

    pub fn new() -> Cache {
        Cache{local_hashes: HashSet::new(), listeners: Vec::new()}
    }

    pub fn register_listener(&mut self, handler: FileListener) {
        self.listeners.push(handler);
    }

    pub fn in_cache (&self, file_info: &FileInfo) -> bool {
        return self.local_hashes.contains(file_info);
    }

    pub fn process_file(&mut self, file_info: FileInfo) {
    
        self.listeners
            .iter()
            .filter(|file_handler| file_handler.condition(&file_info))
            .for_each(|file_handler| file_handler.callback(&file_info));

            self.local_hashes.insert(file_info);
    }
}




