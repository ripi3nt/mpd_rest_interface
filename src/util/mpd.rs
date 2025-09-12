use mpd::{Client, Id, Query, Song, Status};
use serde::Serialize;
use std::{error::Error, net::TcpStream};

#[derive(Serialize)]
pub struct SongData {
    file: String,
    title: Option<String>,
    tags: Vec<(String, String)>
}

pub struct MpdClient {
    client : Client<TcpStream>
}

impl MpdClient {
    pub fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(addr)?;
        let client = Client::new(stream)?;

        Ok(MpdClient { client })
    }

    pub fn search(&mut self, value: &str) -> Result<Vec<Song>, Box<dyn Error>> {
        let mut query = Query::new();
        query.and(mpd::Term::Any, value);
        let full_data = self.client.search(&query, None)?;
        
        /*let filtered_data : Vec<SongData> = full_data.into_iter().map(|data| SongData {
            file : data.file,
            title: data.title,
            tags: data.tags
        }).collect();*/

        Ok(full_data)
    }

    pub fn play(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.play()?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.stop()?;
        Ok(())
    }

    pub fn toggle_play(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.toggle_pause()?;
        Ok(())
    }

    pub fn playback_status(&mut self) -> Result<Status, Box<dyn Error>> {
        Ok(self.client.status()?)
    }

    pub fn add_to_queue(&mut self, path: Song) -> Result<Id , Box<dyn Error>> {
        let id = self.client.push(path)?;
        Ok(id)
    }

    pub fn remove_from_queue(&mut self, pos: u32) -> Result<(), Box<dyn Error>> {
        self.client.delete(pos)?;
        Ok(())
    }

    pub fn list_queue(&mut self) -> Result<Vec<Song>, Box<dyn Error>> {
        Ok(self.client.queue()?)
    }

    pub fn seek_curr(&mut self, time: f64) -> Result<(), Box<dyn Error>> {
        Ok(self.client.rewind(time)?)
    }

    pub fn get_album_art(&mut self, song: Song) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.client.albumart(&song)?)
    }

}
