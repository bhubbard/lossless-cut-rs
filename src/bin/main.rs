use clap::{Parser, Subcommand};
use lossless_cut_rs::{
    load_csv_segments, load_llc_project, parse_timecode, CutEngine, Result,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lossless-cut")]
#[command(author = "Brandon Hubbard <brandon@brandonhubbard.com>")]
#[command(version = "0.0.1")]
#[command(about = "Blazingly fast, zero-overhead Rust engine for lossless video & audio surgery", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Slice a single time segment losslessly from a video
    Trim {
        /// Input video file path
        #[arg(short = 'i', long = "input", required = true)]
        input: PathBuf,

        /// Output trimmed video file path
        #[arg(short = 'o', long = "output", required = true)]
        output: PathBuf,

        /// Cut start time (e.g. 00:01:30 or 90.0)
        #[arg(short = 's', long = "start", default_value = "0")]
        start: String,

        /// Cut end time (e.g. 00:03:00 or 180.0)
        #[arg(short = 'e', long = "end", required = true)]
        end: String,
    },

    /// Batch cut segments from an .llc project file or CSV timeline
    Batch {
        /// Input media file
        #[arg(short = 'i', long = "input", required = true)]
        input: PathBuf,

        /// Path to .llc project file or .csv segment list
        #[arg(short = 'p', long = "project", required = true)]
        project: PathBuf,

        /// Output directory for segment exports
        #[arg(short = 'o', long = "output-dir", default_value = "./cut_output")]
        output_dir: PathBuf,

        /// Automatically concatenate all exported segments into one file
        #[arg(short = 'm', long = "merge")]
        merge: bool,

        /// Custom merged file name
        #[arg(long = "merge-name")]
        merge_name: Option<String>,
    },

    /// Losslessly merge/concatenate multiple video files of the same codec
    Merge {
        /// List of video files to merge in order
        #[arg(short = 'i', long = "inputs", required = true, num_args = 2..)]
        inputs: Vec<PathBuf>,

        /// Output merged video file
        #[arg(short = 'o', long = "output", required = true)]
        output: PathBuf,
    },

    /// Extract an audio track losslessly without transcoding
    ExtractAudio {
        /// Input video file
        #[arg(short = 'i', long = "input", required = true)]
        input: PathBuf,

        /// Output audio file (e.g. out.m4a, out.opus, out.mp3)
        #[arg(short = 'o', long = "output", required = true)]
        output: PathBuf,

        /// Audio track index (0 = first audio track)
        #[arg(short = 't', long = "track", default_value = "0")]
        track: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let engine = CutEngine::new();

    match cli.command {
        Commands::Trim {
            input,
            output,
            start,
            end,
        } => {
            let start_sec = parse_timecode(&start)?;
            let end_sec = parse_timecode(&end)?;
            println!("Trimming '{}' from {:.3}s to {:.3}s...", input.display(), start_sec, end_sec);
            engine.cut_segment(&input, &output, start_sec, end_sec).await?;
            println!("✓ Export complete: {}", output.display());
        }

        Commands::Batch {
            input,
            project,
            output_dir,
            merge,
            merge_name,
        } => {
            let segments = if project.extension().and_then(|e| e.to_str()) == Some("csv") {
                load_csv_segments(&project)?
            } else {
                let proj = load_llc_project(&project)?;
                proj.cut_segments
            };

            println!("Loaded {} segments from project '{}'", segments.len(), project.display());
            let exported = engine
                .process_segments(&input, &segments, &output_dir, merge, merge_name.as_deref())
                .await?;

            for p in exported {
                println!("✓ Output: {}", p.display());
            }
        }

        Commands::Merge { inputs, output } => {
            println!("Merging {} video segments...", inputs.len());
            engine.merge_files(&inputs, &output).await?;
            println!("✓ Merge complete: {}", output.display());
        }

        Commands::ExtractAudio {
            input,
            output,
            track,
        } => {
            let args = lossless_cut_rs::build_extract_audio_args(&input, &output, track);
            let status = tokio::process::Command::new(&engine.ffmpeg_path)
                .args(&args)
                .status()
                .await?;

            if !status.success() {
                eprintln!("Failed to extract audio stream.");
            } else {
                println!("✓ Audio track {} extracted to {}", track, output.display());
            }
        }
    }

    Ok(())
}
