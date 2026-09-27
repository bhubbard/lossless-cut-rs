use crate::timecode::format_timecode;
use std::path::{Path, PathBuf};

/// Builds arguments for a lossless stream-copy slice.
pub fn build_lossless_cut_args(
    input: &Path,
    output: &Path,
    start: f64,
    end: f64,
    avoid_negative_ts: bool,
) -> Vec<String> {
    let mut args = vec![
        "-hide_banner".to_string(),
        "-y".to_string(),
        "-ss".to_string(),
        format_timecode(start),
        "-to".to_string(),
        format_timecode(end),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-c".to_string(),
        "copy".to_string(),
        "-map".to_string(),
        "0".to_string(),
    ];

    if avoid_negative_ts {
        args.push("-avoid_negative_ts".to_string());
        args.push("make_zero".to_string());
    }

    args.push(output.to_string_lossy().to_string());
    args
}

/// Builds arguments for extracting a specific audio track stream losslessly.
pub fn build_extract_audio_args(
    input: &Path,
    output: &Path,
    audio_index: usize,
) -> Vec<String> {
    vec![
        "-hide_banner".to_string(),
        "-y".to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vn".to_string(),
        "-map".to_string(),
        format!("0:a:{}", audio_index),
        "-c:a".to_string(),
        "copy".to_string(),
        output.to_string_lossy().to_string(),
    ]
}

/// Builds arguments for extracting a subtitle track.
pub fn build_extract_subtitle_args(
    input: &Path,
    output: &Path,
    subtitle_index: usize,
) -> Vec<String> {
    vec![
        "-hide_banner".to_string(),
        "-y".to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vn".to_string(),
        "-an".to_string(),
        "-map".to_string(),
        format!("0:s:{}", subtitle_index),
        "-c:s".to_string(),
        "copy".to_string(),
        output.to_string_lossy().to_string(),
    ]
}

/// Builds arguments for the FFmpeg concat demuxer.
pub fn build_concat_args(concat_list_file: &Path, output: &Path) -> Vec<String> {
    vec![
        "-hide_banner".to_string(),
        "-y".to_string(),
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-i".to_string(),
        concat_list_file.to_string_lossy().to_string(),
        "-c".to_string(),
        "copy".to_string(),
        output.to_string_lossy().to_string(),
    ]
}

/// Generates the concat list text file contents for FFmpeg concat demuxer.
pub fn generate_concat_list(segment_paths: &[PathBuf]) -> String {
    let mut out = String::new();
    for p in segment_paths {
        out.push_str(&format!("file '{}'\n", p.display()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_lossless_cut_args() {
        let input = Path::new("input.mp4");
        let output = Path::new("out.mp4");
        let args = build_lossless_cut_args(input, output, 10.0, 30.5, true);

        assert!(args.contains(&"-ss".to_string()));
        assert!(args.contains(&"00:00:10.000".to_string()));
        assert!(args.contains(&"-to".to_string()));
        assert!(args.contains(&"00:00:30.500".to_string()));
        assert!(args.contains(&"copy".to_string()));
        assert!(args.contains(&"make_zero".to_string()));
    }

    #[test]
    fn test_generate_concat_list() {
        let segs = vec![
            PathBuf::from("/path/seg1.mp4"),
            PathBuf::from("/path/seg2.mp4"),
        ];
        let list = generate_concat_list(&segs);
        assert_eq!(list, "file '/path/seg1.mp4'\nfile '/path/seg2.mp4'\n");
    }
}
