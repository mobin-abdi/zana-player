use lofty::file::AudioFile;
use lofty::prelude::*;
use lofty::probe::Probe;

use rodio::{Decoder, Source};

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Song {
    pub id: usize,
    pub title: String,
    pub path: PathBuf,
}

pub fn scan_music_directory(path: PathBuf) -> Vec<Song> {
    let Ok(entries) = fs::read_dir(path) else {
        return Vec::new();
    };

    let mut songs = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };

        match extension.to_lowercase().as_str() {
            "mp3" | "m4a" | "flac" | "wav" | "ogg" | "aac" | "opus" => {
                let title = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                songs.push(Song {
                    id: 0,
                    title,
                    path,
                });
            }
            _ => {}
        }
    }

    songs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

    for (id, song) in songs.iter_mut().enumerate() {
        song.id = id;
    }

    songs
}

pub fn load_metadata(path: &PathBuf) -> (String, Option<String>, Option<String>, f32) {
    let default_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown Title".to_string());

    if let Ok(tagged_file) = Probe::open(path).and_then(|p| p.read()) {
        let properties = tagged_file.properties();
        let mut duration = properties.duration().as_secs_f32();

        if duration <= 0.0 {
            if let Ok(file) = File::open(path) {
                if let Ok(decoder) = Decoder::new(BufReader::new(file)) {
                    duration = decoder
                        .total_duration()
                        .map(|d| d.as_secs_f32())
                        .unwrap_or(0.0);
                }
            }
        }

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag());

        let title = tag
            .and_then(|t| t.title())
            .map(|t| t.to_string())
            .unwrap_or(default_title);

        let artist = tag.and_then(|t| t.artist()).map(|a| a.to_string());
        let album = tag.and_then(|t| t.album()).map(|a| a.to_string());

        return (title, artist, album, duration);
    }

    (default_title, None, None, 0.0)
}
