use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "voxscribe", about = "Record and transcribe audio locally")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Interactive recording session
    Record(RecordArgs),
    /// Transcribe an existing audio file
    Transcribe(TranscribeArgs),
    /// Download the whisper model
    DownloadModel(DownloadModelArgs),
}

#[derive(clap::Args, Debug)]
pub struct RecordArgs {
    /// Non-interactive mode (Ctrl+C to stop, no auto-transcribe)
    #[arg(long)]
    pub no_interact: bool,

    /// Override transcription output path
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Override WAV file output directory
    #[arg(long)]
    pub audio_dir: Option<PathBuf>,

    /// Keep the WAV file after transcription
    #[arg(long)]
    pub keep_audio: bool,

    /// Optimize for a single speaker (disables speaker turn detection)
    #[arg(long)]
    pub single_speaker: bool,

    /// Language code (e.g. "en", "sv"). Auto-detects if omitted
    #[arg(long)]
    pub language: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct TranscribeArgs {
    /// Path to the audio file to transcribe
    pub file: PathBuf,

    /// Override transcription output path
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Optimize for a single speaker (disables speaker turn detection)
    #[arg(long)]
    pub single_speaker: bool,

    /// Language code (e.g. "en", "sv"). Auto-detects if omitted
    #[arg(long)]
    pub language: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct DownloadModelArgs {
    /// Whisper model variant to download
    #[arg(long)]
    pub model: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Cli {
        Cli::try_parse_from(args).unwrap()
    }

    #[test]
    fn parse_record_defaults() {
        let cli = parse(&["voxscribe", "record"]);
        match cli.command {
            Command::Record(args) => {
                assert!(!args.no_interact);
                assert!(args.output.is_none());
                assert!(args.audio_dir.is_none());
                assert!(!args.keep_audio);
                assert!(!args.single_speaker);
            }
            _ => panic!("expected Record command"),
        }
    }

    #[test]
    fn parse_record_single_speaker() {
        let cli = parse(&["voxscribe", "record", "--single-speaker"]);
        match cli.command {
            Command::Record(args) => assert!(args.single_speaker),
            _ => panic!("expected Record command"),
        }
    }

    #[test]
    fn parse_transcribe_single_speaker() {
        let cli = parse(&[
            "voxscribe",
            "transcribe",
            "recording.wav",
            "--single-speaker",
        ]);
        match cli.command {
            Command::Transcribe(args) => assert!(args.single_speaker),
            _ => panic!("expected Transcribe command"),
        }
    }

    #[test]
    fn parse_record_with_language() {
        let cli = parse(&["voxscribe", "record", "--language", "sv"]);
        match cli.command {
            Command::Record(args) => assert_eq!(args.language.unwrap(), "sv"),
            _ => panic!("expected Record command"),
        }
    }

    #[test]
    fn parse_transcribe_with_language() {
        let cli = parse(&[
            "voxscribe",
            "transcribe",
            "recording.wav",
            "--language",
            "en",
        ]);
        match cli.command {
            Command::Transcribe(args) => assert_eq!(args.language.unwrap(), "en"),
            _ => panic!("expected Transcribe command"),
        }
    }

    #[test]
    fn parse_record_language_defaults_to_none() {
        let cli = parse(&["voxscribe", "record"]);
        match cli.command {
            Command::Record(args) => assert!(args.language.is_none()),
            _ => panic!("expected Record command"),
        }
    }

    #[test]
    fn parse_record_all_flags() {
        let cli = parse(&[
            "voxscribe",
            "record",
            "--no-interact",
            "--output",
            "/tmp/out.txt",
            "--audio-dir",
            "/tmp/audio",
            "--keep-audio",
        ]);
        match cli.command {
            Command::Record(args) => {
                assert!(args.no_interact);
                assert_eq!(args.output.unwrap(), PathBuf::from("/tmp/out.txt"));
                assert_eq!(args.audio_dir.unwrap(), PathBuf::from("/tmp/audio"));
                assert!(args.keep_audio);
            }
            _ => panic!("expected Record command"),
        }
    }

    #[test]
    fn parse_transcribe() {
        let cli = parse(&["voxscribe", "transcribe", "recording.wav"]);
        match cli.command {
            Command::Transcribe(args) => {
                assert_eq!(args.file, PathBuf::from("recording.wav"));
                assert!(args.output.is_none());
            }
            _ => panic!("expected Transcribe command"),
        }
    }

    #[test]
    fn parse_transcribe_with_output() {
        let cli = parse(&[
            "voxscribe",
            "transcribe",
            "recording.wav",
            "--output",
            "/tmp/out.txt",
        ]);
        match cli.command {
            Command::Transcribe(args) => {
                assert_eq!(args.output.unwrap(), PathBuf::from("/tmp/out.txt"));
            }
            _ => panic!("expected Transcribe command"),
        }
    }

    #[test]
    fn parse_download_model_default() {
        let cli = parse(&["voxscribe", "download-model"]);
        match cli.command {
            Command::DownloadModel(args) => {
                assert!(args.model.is_none());
            }
            _ => panic!("expected DownloadModel command"),
        }
    }

    #[test]
    fn parse_download_model_with_name() {
        let cli = parse(&["voxscribe", "download-model", "--model", "tiny"]);
        match cli.command {
            Command::DownloadModel(args) => {
                assert_eq!(args.model.unwrap(), "tiny");
            }
            _ => panic!("expected DownloadModel command"),
        }
    }

    #[test]
    fn missing_subcommand_is_error() {
        assert!(Cli::try_parse_from(["voxscribe"]).is_err());
    }

    #[test]
    fn transcribe_missing_file_is_error() {
        assert!(Cli::try_parse_from(["voxscribe", "transcribe"]).is_err());
    }
}
