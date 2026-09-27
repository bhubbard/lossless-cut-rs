use lossless_cut_rs::{
    build_extract_audio_args, build_extract_subtitle_args, build_lossless_cut_args,
    format_timecode, generate_concat_list, load_csv_segments, load_llc_project, parse_timecode,
    save_csv_segments, save_llc_project, ProjectFile, Segment,
};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[test]
fn test_timecode_conversions() {
    assert_eq!(parse_timecode("00:00:00").unwrap(), 0.0);
    assert_eq!(parse_timecode("00:01:23.450").unwrap(), 83.45);
    assert_eq!(parse_timecode("02:00:00").unwrap(), 7200.0);
    assert_eq!(parse_timecode("15.75").unwrap(), 15.75);

    assert_eq!(format_timecode(83.45), "00:01:23.450");
    assert_eq!(format_timecode(7200.0), "02:00:00.000");
}

#[test]
fn test_project_file_serialization() {
    let seg1 = Segment::new(12.5, 45.0).with_label("Scene One");
    let seg2 = Segment::new(60.0, 95.5).with_label("Scene Two");
    let project = ProjectFile::new("video_recording.mov", vec![seg1, seg2]);

    let tmp = NamedTempFile::new().unwrap();
    save_llc_project(tmp.path(), &project).unwrap();

    let reloaded = load_llc_project(tmp.path()).unwrap();
    assert_eq!(reloaded.version, 1);
    assert_eq!(reloaded.media_file_name, "video_recording.mov");
    assert_eq!(reloaded.cut_segments.len(), 2);
    assert_eq!(reloaded.cut_segments[0].label, Some("Scene One".to_string()));
    assert_eq!(reloaded.cut_segments[1].duration(), 35.5);
}

#[test]
fn test_csv_timeline_import_export() {
    let segs = vec![
        Segment::new(0.0, 30.0).with_label("Intro"),
        Segment::new(100.5, 200.25).with_label("Outro"),
    ];

    let tmp = NamedTempFile::new().unwrap();
    save_csv_segments(tmp.path(), &segs).unwrap();

    let reloaded = load_csv_segments(tmp.path()).unwrap();
    assert_eq!(reloaded.len(), 2);
    assert_eq!(reloaded[0].start, 0.0);
    assert_eq!(reloaded[0].end, 30.0);
    assert_eq!(reloaded[1].start, 100.5);
}

#[test]
fn test_ffmpeg_command_generation() {
    let in_p = Path::new("/videos/sample.mp4");
    let out_p = Path::new("/videos/sample_cut.mp4");

    let cut_args = build_lossless_cut_args(in_p, out_p, 15.0, 45.0, true);
    assert!(cut_args.contains(&"-c".to_string()));
    assert!(cut_args.contains(&"copy".to_string()));
    assert!(cut_args.contains(&"-avoid_negative_ts".to_string()));

    let audio_args = build_extract_audio_args(in_p, Path::new("audio.aac"), 1);
    assert!(audio_args.contains(&"0:a:1".to_string()));
    assert!(audio_args.contains(&"-vn".to_string()));

    let sub_args = build_extract_subtitle_args(in_p, Path::new("sub.srt"), 0);
    assert!(sub_args.contains(&"0:s:0".to_string()));
    assert!(sub_args.contains(&"-an".to_string()));

    let concat_list = generate_concat_list(&[
        PathBuf::from("/cuts/part1.mp4"),
        PathBuf::from("/cuts/part2.mp4"),
    ]);
    assert!(concat_list.contains("file '/cuts/part1.mp4'"));
    assert!(concat_list.contains("file '/cuts/part2.mp4'"));
}
