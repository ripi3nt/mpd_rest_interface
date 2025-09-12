use std::{collections::HashMap, fs, path::PathBuf};
use lofty::{ file::TaggedFileExt, read_from_path, tag::{Accessor, TagType}};

pub fn list_albums(dir: &String) -> Result<HashMap<String, String> , Box<dyn std::error::Error>> {
    let mut files = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let mut file = entry?;

        if file.metadata()?.is_dir() {
            file = fs::read_dir(file.path()).unwrap().into_iter().find(|_e| true).unwrap()?;
        }

        if !file.path().display().to_string().ends_with(".flac") {
            continue
        }

        let file = read_from_path(file.path())?;
        match file.tag(TagType::VorbisComments) {
            None => {println!("Error getting file tags");}
            Some(name) => {files.insert(name.album().unwrap().to_string(), name.album().unwrap().to_string());}
        }
    };

    Ok(files)

}

//use redis database for faster retrieval

pub fn load_albums_db() {

}

pub fn list_songs_mpd() {
    
}


