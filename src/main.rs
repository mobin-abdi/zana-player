mod music;
mod player;

slint::include_modules!();

use music::{load_metadata, scan_music_directory};
use player::AudioPlayer;

use slint::ComponentHandle;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let audio_player = Rc::new(AudioPlayer::new().expect("Failed to initialize audio player"));
    let songs = Rc::new(scan_music_directory(PathBuf::from("/home/mobin/Music")));

    let titles: Vec<slint::SharedString> = songs.iter().map(|s| s.title.clone().into()).collect();
    let artists: Vec<slint::SharedString> = songs.iter().map(|_| "Unknown Artist".into()).collect();
    let ids: Vec<i32> = songs.iter().map(|s| s.id as i32).collect();

    window.set_song_titles(Rc::new(slint::VecModel::from(titles)).into());
    window.set_song_artists(Rc::new(slint::VecModel::from(artists)).into());
    window.set_song_ids(Rc::new(slint::VecModel::from(ids)).into());

    let play_song_by_id = Rc::new({
        let songs = songs.clone();
        let audio_player = audio_player.clone();
        let window_weak = window.as_weak();

        move |id: i32| {
            let Some(window) = window_weak.upgrade() else {
                return;
            };
            let Some(song) = songs.iter().find(|s| s.id as i32 == id) else {
                return;
            };

            if let Err(error) = audio_player.play(&song.path) {
                eprintln!("Playback error: {error}");
                return;
            }

            let (title, artist, album, _) = load_metadata(&song.path);

            let duration = audio_player.duration(&song.path).unwrap_or(0.0);

            window.set_is_paused(false);
            window.set_current_song_id(id);
            window.set_current_song_title(title.into());
            window
                .set_current_song_artist(artist.unwrap_or_else(|| "Unknown Artist".into()).into());
            window.set_current_song_album(album.unwrap_or_else(|| "Unknown Album".into()).into());

            window.set_current_song_duration(duration);
            window.set_current_song_position(0.0);

            window.set_current_page(2);
        }
    });

    let play_next = {
        let play_song = play_song_by_id.clone();
        let window_weak = window.as_weak();
        let songs_count = songs.len();

        move || {
            let Some(window) = window_weak.upgrade() else {
                return;
            };
            let current_id = window.get_current_song_id();
            if current_id >= 0 && songs_count > 0 {
                let next_id = (current_id + 1) % songs_count as i32;
                play_song(next_id);
            }
        }
    };

    let play_song_cloned = play_song_by_id.clone();
    window.on_song_selected(move |id| {
        play_song_cloned(id);
    });

    let songs_for_search = songs.clone();
    let window_weak_for_search = window.as_weak();

    window.on_search_changed(move |query| {
        let Some(window) = window_weak_for_search.upgrade() else {
            return;
        };

        let query = query.trim().to_lowercase();

        let filtered_songs = songs_for_search
            .iter()
            .filter(|song| query.is_empty() || song.title.to_lowercase().contains(&query));

        let mut titles = Vec::new();
        let mut artists = Vec::new();
        let mut ids = Vec::new();

        for song in filtered_songs {
            titles.push(song.title.clone().into());
            artists.push("Unknown Artist".into());
            ids.push(song.id as i32);
        }

        window.set_song_titles(Rc::new(slint::VecModel::from(titles)).into());

        window.set_song_artists(Rc::new(slint::VecModel::from(artists)).into());

        window.set_song_ids(Rc::new(slint::VecModel::from(ids)).into());
    });

    let play_next_cloned = play_next.clone();
    window.on_next_clicked(move || {
        play_next_cloned();
    });

    let play_song_cloned = play_song_by_id.clone();
    let window_weak = window.as_weak();
    let songs_count = songs.len();

    window.on_previous_clicked(move || {
        let Some(window) = window_weak.upgrade() else {
            return;
        };
        let current_id = window.get_current_song_id();
        if current_id >= 0 && songs_count > 0 {
            let prev_id = if current_id == 0 {
                (songs_count - 1) as i32
            } else {
                current_id - 1
            };
            play_song_cloned(prev_id);
        }
    });

    let audio_player_for_seek = audio_player.clone();
    let window_weak_seek = window.as_weak();

    window.on_seek_requested(move |new_pos| {
        audio_player_for_seek.seek(Duration::from_secs_f32(new_pos));

        if let Some(window) = window_weak_seek.upgrade() {
            window.set_current_song_position(new_pos);
        }
    });

    let timer = slint::Timer::default();
    let window_weak = window.as_weak();
    let audio_player_for_timer = audio_player.clone();
    let play_next_for_timer = play_next.clone();

    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(250),
        move || {
            let Some(window) = window_weak.upgrade() else {
                return;
            };

            // فقط زمانی آپدیت کن که موزیک در حال پخش باشه
            if !audio_player_for_timer.is_paused() {
                let pos = audio_player_for_timer.position().as_secs_f32();
                let duration = window.get_current_song_duration();

                window.set_current_song_position(pos);

                if duration > 0.0 && pos >= (duration - 0.5) {
                    play_next_for_timer();
                }
            }
        },
    );

    let window_weak = window.as_weak();
    let audio_player_for_play_pause = audio_player.clone();

    window.on_play_pause_clicked(move || {
        let Some(window) = window_weak.upgrade() else {
            return;
        };
        if audio_player_for_play_pause.is_paused() {
            audio_player_for_play_pause.resume();
            window.set_is_paused(false);
        } else {
            audio_player_for_play_pause.pause();
            window.set_is_paused(true);
        }
    });

    window.run()?;

    drop(timer);

    Ok(())
}
