use crate::error::{CutError, Result};
use crate::ffmpeg::{build_concat_args, build_lossless_cut_args, generate_concat_list};
use crate::types::Segment;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub struct CutEngine {
    pub ffmpeg_path: String,
}

impl Default for CutEngine {
    fn default() -> Self {
        Self {
            ffmpeg_path: "ffmpeg".to_string(),
        }
    }
}

impl CutEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes a single lossless cut.
    pub async fn cut_segment(
        &self,
        input: &Path,
        output: &Path,
        start: f64,
        end: f64,
    ) -> Result<()> {
        if start >= end {
            return Err(CutError::InvalidRange { start, end });
        }

        let args = build_lossless_cut_args(input, output, start, end, true);
        self.run_ffmpeg(&args).await
    }

    /// Slices multiple segments and optionally merges them into a single final file.
    pub async fn process_segments(
        &self,
        input: &Path,
        segments: &[Segment],
        output_dir: &Path,
        merge: bool,
        merged_output_name: Option<&str>,
    ) -> Result<Vec<PathBuf>> {
        tokio::fs::create_dir_all(output_dir).await?;

        let stem = input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("cut_export");
        let ext = input
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("mp4");

        let pb = ProgressBar::new(segments.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] {pos}/{len} segments ({msg})")
                .unwrap(),
        );

        let mut cut_paths = Vec::new();

        for (idx, seg) in segments.iter().enumerate() {
            let label_part = seg
                .label
                .as_ref()
                .map(|l| format!("_{}", l.replace(' ', "_")))
                .unwrap_or_default();

            let out_file_name = format!("{}_seg{}{}.{}", stem, idx + 1, label_part, ext);
            let out_path = output_dir.join(out_file_name);

            pb.set_message(format!("Cutting segment {}", idx + 1));
            self.cut_segment(input, &out_path, seg.start, seg.end).await?;
            cut_paths.push(out_path);
            pb.inc(1);
        }

        pb.finish_with_message("All segments cut successfully");

        if merge && cut_paths.len() > 1 {
            let final_name = merged_output_name
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{}_merged.{}", stem, ext));
            let final_path = output_dir.join(final_name);

            println!("Merging {} segments into {}...", cut_paths.len(), final_path.display());
            self.merge_files(&cut_paths, &final_path).await?;
            return Ok(vec![final_path]);
        }

        Ok(cut_paths)
    }

    /// Concatenates multiple video files of identical codec/streams into a single file.
    pub async fn merge_files(&self, inputs: &[PathBuf], output: &Path) -> Result<()> {
        let concat_txt = generate_concat_list(inputs);
        let temp_list_path = output.with_extension("concat.txt");
        tokio::fs::write(&temp_list_path, concat_txt).await?;

        let args = build_concat_args(&temp_list_path, output);
        let res = self.run_ffmpeg(&args).await;
        tokio::fs::remove_file(&temp_list_path).await.ok();
        res
    }

    async fn run_ffmpeg(&self, args: &[String]) -> Result<()> {
        let output = Command::new(&self.ffmpeg_path)
            .args(args)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(CutError::FfmpegExecution {
                code: output.status.code(),
                stderr,
            });
        }

        Ok(())
    }
}
