# ZanaPlayer 🎵

A simple and lightweight music player for Linux, built with **Rust** and **Slint**.

ZanaPlayer is a personal project focused on building a clean, fast, and minimal desktop music player while exploring Rust audio playback and Slint UI development.

> 🚧 ZanaPlayer is still under development.

## ✨ Features

* 🎵 Local music library
* 🔎 Search songs
* ▶️ Play / Pause
* ⏮️ Previous / Next track
* ⏱️ Playback timeline
* 🎚️ Seek through songs
* 🏷️ Read song metadata
* 🖥️ Clean desktop interface
* 🦀 Built with Rust

## 🛠️ Tech Stack

* **Rust** — application and audio logic
* **Slint** — graphical user interface
* **Rodio** — audio playback
* **Lofty** — audio metadata

## 📁 Supported Formats

ZanaPlayer currently scans for:

* MP3
* M4A
* FLAC
* WAV
* OGG
* AAC
* Opus

Actual playback support may depend on the codecs supported by the underlying audio backend.

## 🚀 Running

### Requirements

* Linux
* Rust toolchain
* Cargo

Clone the repository:

```bash
git clone https://github.com/mobin-abdi/zana-player.git
cd zana-player
```

Run:

```bash
cargo run
```

Build a release version:

```bash
cargo build --release
```

## 🎨 Design

ZanaPlayer is intentionally kept simple. The interface focuses on the things I personally need when listening to local music rather than trying to become an all-in-one music management application.

## 🗺️ Roadmap

Some things I may add in the future:

* [ ] Album artwork
* [ ] Volume control
* [ ] Shuffle
* [ ] Repeat
* [ ] Better playlist support
* [ ] More metadata support
* [ ] Improved library management
* [ ] Configuration/settings

## 🤝 Contributing

This is primarily a personal project, but suggestions, bug reports, and contributions are welcome.

## 📄 License

This project is licensed under the MIT License.
