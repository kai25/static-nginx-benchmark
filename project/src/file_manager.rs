use std::fs;

use bytes::Bytes;
use std::collections::HashMap;

use crate::config::ContentConfig;
use std::sync::Arc;

struct File {
    path: Option<String>,
    url: Option<String>,
    counter: i32,
    bytes: Option<Bytes>
}

fn load_file(path: &str) -> Bytes {
    println!("Loading from disk {}", path);
    Bytes::from(fs::read(path).unwrap())
}

impl File {
    fn load_from_fs(&self, path: &str) -> Bytes {
        match &self.bytes {
            Some(bytes) => bytes.to_owned(),
            None => load_file(path),
        }
    }

    fn load_from_url(url: &str) -> Bytes {
        Bytes::from("result")
    }

    fn get_content(&self) -> Bytes {
        match (&self.path, &self.url) {
            (Some(path), None) => self.load_from_fs(path),
            _ => Bytes::from("q"),
        }
    }
}

pub struct FileManager {
    memory_usage: i32,
    files: HashMap<String, File>
}

impl FileManager {
    pub fn from_config(content_config: &ContentConfig) -> Self {
        let mut manager = FileManager {
            memory_usage: 0,
            files: HashMap::new(),
        };

        content_config.files.iter()
            .for_each(|(route, file_config)| {
                let bytes = match (&file_config.path, &file_config.in_memory) {
                    (Some(path), true) => Some(load_file(path)),
                    _ => None,
                };

                let file = File {
                    path: file_config.path.to_owned(),
                    url: file_config.url.to_owned(),
                    counter: 0,
                    bytes: bytes,
                };

                manager.files.insert(route.to_owned(), file);
            });

        manager
    }

    pub fn get(&self, route: &str) -> Option<Bytes> {
        self.files.get(route)
            .map(|file| file.get_content())
    }
}