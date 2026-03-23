# Notetaker

Record and transcribe audio locally using [whisper.cpp](https://github.com/ggerganov/whisper.cpp). No cloud services required.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) — install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Install

```sh
git clone <repo-url>
cd notetaker
cargo install --path .
```

Then download the whisper model:

```sh
notetaker download-model
```

## Usage

```sh
notetaker record                # interactive recording + transcription
notetaker record --language sv  # pin language (auto-detects by default)
notetaker record --single-speaker  # optimize for a single speaker
notetaker transcribe audio.wav  # transcribe an existing file
```

Transcriptions are saved to `~/transcriptions/` by default.

## Development

### Prerequisites

- [just](https://github.com/casey/just) — install via `brew install just` (macOS) or see [installation docs](https://github.com/casey/just#installation)

### Setup

```sh
just init
```

`just init` installs required Rust components (rustfmt, clippy) and sets up git hooks.

### Commands

| Command              | Description                            |
|----------------------|----------------------------------------|
| `just install`       | Install to `~/.cargo/bin/`             |
| `just build`         | Compile the project (debug)            |
| `just build-release` | Compile optimized release build        |
| `just run <args>`    | Build and run (e.g. `just run record`) |
| `just test`          | Run all tests                          |
| `just fmt`           | Format code with rustfmt               |
| `just lint`          | Run clippy                             |
| `just check`         | Run fmt + clippy + tests               |
