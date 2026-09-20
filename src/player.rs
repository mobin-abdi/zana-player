use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::fs::File;
use std::path::Path;
use std::time::Duration;

pub struct AudioPlayer {
    _stream: rodio::MixerDeviceSink,
    player: Player,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(stream.mixer());

        Ok(Self {
            _stream: stream,
            player,
        })
    }

    pub fn play(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let decoder = Decoder::try_from(file)?;

        self.player.clear();
        self.player.append(decoder);
        self.player.play();

        Ok(())
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn seek(&self, position: Duration) {
        if let Err(error) = self.player.try_seek(position) {
            eprintln!("Seek error: {error}");
        }
    }
    pub fn duration(&self, path: &Path) -> Result<f32, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let decoder = Decoder::try_from(file)?;

        Ok(decoder
            .total_duration()
            .map(|duration| duration.as_secs_f32())
            .unwrap_or(0.0))
    }
}
