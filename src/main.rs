mod cli;
mod config;
mod download;
mod output;
mod recorder;
mod transcriber;
mod ui;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use config::AppConfig;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    whisper_rs::install_logging_hooks();

    let cli = Cli::parse();
    let config = AppConfig::load()?.merge_cli(&cli);

    match cli.command {
        Command::Record(args) => cmd_record(&config, &args),
        Command::Transcribe(args) => cmd_transcribe(&config, &args),
        Command::DownloadModel(args) => cmd_download_model(&config, &args),
    }
}

fn cmd_record(config: &AppConfig, args: &cli::RecordArgs) -> Result<()> {
    let model_path = config.resolved_model_path();
    if !args.no_interact && !model_path.exists() {
        anyhow::bail!(
            "Whisper model not found at {}. Run `voxscribe download-model` first.",
            model_path.display()
        );
    }

    let recorder = recorder::Recorder::new(config)?;
    let wav_path = if args.no_interact {
        recorder::run_non_interactive(recorder)?
    } else {
        ui::run_interactive(recorder)?
    };

    if args.no_interact {
        let dest = output::resolve_audio_path(config, args);
        std::fs::rename(&wav_path, &dest)?;
        println!("{}", dest.display());
    } else {
        let duration = recorder::wav_duration(&wav_path)?;
        if duration < 1.0 {
            eprintln!("Recording too short (< 1 second), skipping transcription.");
            std::fs::remove_file(&wav_path).ok();
            return Ok(());
        }

        let text = transcribe_with_spinner(
            &model_path,
            &wav_path,
            args.single_speaker,
            args.language.as_deref(),
        )?;
        let out_path = output::resolve_output_path(config, args.output.as_deref());
        output::write_transcription(&out_path, &text)?;
        println!("{}", out_path.display());

        if args.keep_audio {
            let audio_dest = output::resolve_audio_path(config, args);
            std::fs::rename(&wav_path, &audio_dest)?;
            println!("Audio saved to {}", audio_dest.display());
        } else {
            std::fs::remove_file(&wav_path).ok();
        }
    }

    Ok(())
}

fn cmd_transcribe(config: &AppConfig, args: &cli::TranscribeArgs) -> Result<()> {
    let model_path = config.resolved_model_path();
    if !model_path.exists() {
        anyhow::bail!(
            "Whisper model not found at {}. Run `voxscribe download-model` first.",
            model_path.display()
        );
    }

    let wav_path = &args.file;
    if !wav_path.exists() {
        anyhow::bail!("Audio file not found: {}", wav_path.display());
    }

    let text = transcribe_with_spinner(
        &model_path,
        wav_path,
        args.single_speaker,
        args.language.as_deref(),
    )?;
    let out_path = output::resolve_output_path(config, args.output.as_deref());
    output::write_transcription(&out_path, &text)?;
    println!("{}", out_path.display());

    Ok(())
}

fn cmd_download_model(config: &AppConfig, args: &cli::DownloadModelArgs) -> Result<()> {
    let model_name = args.model.as_deref().unwrap_or(&config.model);
    download::download_model(model_name, &config.resolved_model_dir())?;
    Ok(())
}

fn transcribe_with_spinner(
    model_path: &std::path::Path,
    wav_path: &std::path::Path,
    single_speaker: bool,
    language: Option<&str>,
) -> Result<String> {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .template("{spinner:.green} Transcribing...")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = transcriber::transcribe(model_path, wav_path, single_speaker, language);

    match &result {
        Ok(_) => spinner.finish_and_clear(),
        Err(_) => spinner.finish_with_message("Transcription failed"),
    }

    result
}
