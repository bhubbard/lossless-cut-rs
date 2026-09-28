//! Analytical and Temporal Accuracy Verification Tests for lossless-cut-rs
//!
//! Validates:
//! 1. Sub-millisecond timecode format <-> parse continuous roundtrip precision.
//! 2. Timeline segment partition conservation and non-negative duration bounds.
//! 3. Lossless FFmpeg command stream-copy invariance (-c copy, -avoid_negative_ts make_zero).
//! 4. Upstream LLC JSON project specification fidelity.

use lossless_cut_rs::{
    build_lossless_cut_args, format_timecode, parse_timecode, ProjectFile, Segment,
};
use std::path::Path;

#[test]
fn test_timecode_continuous_roundtrip_precision() {
    let test_seconds = [
        0.000, 0.001, 0.500, 1.234, 15.750, 59.999, 60.000, 3599.999, 3600.000,
        3723.456, 86399.999,
    ];

    for &secs in &test_seconds {
        let formatted = format_timecode(secs);
        let parsed = parse_timecode(&formatted).expect("timecode parsing failed");
        let error = (parsed - secs).abs();

        assert!(
            error <= 0.001,
            "Timecode precision drift at {}s: formatted '{}', parsed {}, err {}",
            secs,
            formatted,
            parsed,
            error
        );
    }
}

#[test]
fn test_timeline_segment_partition_conservation() {
    // Partition a 1-hour timeline (3600.0s) into 5 contiguous slices
    let cuts = [0.0, 450.5, 1200.0, 2100.25, 3000.0, 3600.0];
    let mut total_duration = 0.0;

    for i in 0..cuts.len() - 1 {
        let seg = Segment::new(cuts[i], cuts[i + 1]);
        assert_eq!(seg.duration(), cuts[i + 1] - cuts[i]);
        total_duration += seg.duration();
    }

    assert!(
        (total_duration - 3600.0).abs() < 1e-6,
        "Total timeline duration not conserved: got {}, expected 3600.0",
        total_duration
    );

    // Inverted range duration must be clamped non-negative
    let inverted = Segment::new(100.0, 50.0);
    assert_eq!(inverted.duration(), 0.0);
}

#[test]
fn test_lossless_cut_ffmpeg_command_invariance() {
    let in_file = Path::new("/media/raw_recording.mp4");
    let out_file = Path::new("/media/cut_output.mp4");
    let start = 14.5;
    let end = 89.2;

    let args = build_lossless_cut_args(in_file, out_file, start, end, true);

    // Invariant 1: Must specify bit-exact stream copy without transcoding
    let c_idx = args.iter().position(|r| r == "-c").expect("must contain -c");
    assert_eq!(args[c_idx + 1], "copy", "Must use stream copy -c copy");

    // Invariant 2: Must prevent negative presentation timestamps (PTS)
    let ts_idx = args
        .iter()
        .position(|r| r == "-avoid_negative_ts")
        .expect("must contain -avoid_negative_ts");
    assert_eq!(
        args[ts_idx + 1],
        "make_zero",
        "Must normalize timestamps with make_zero"
    );

    // Invariant 3: Fast seek -ss before -i
    let ss_idx = args.iter().position(|r| r == "-ss").expect("must contain -ss");
    let i_idx = args.iter().position(|r| r == "-i").expect("must contain -i");
    assert!(
        ss_idx < i_idx,
        "Fast seek -ss must precede -i for packet-level demux"
    );
}

#[test]
fn test_llc_schema_spec_fidelity() {
    let segs = vec![
        Segment::new(5.0, 25.0).with_label("Opening Hook"),
        Segment::new(30.0, 90.0).with_label("Demo"),
    ];
    let project = ProjectFile::new("stream.mkv", segs);

    let json_str = serde_json::to_string_pretty(&project).expect("serialize project");
    let json_val: serde_json::Value = serde_json::from_str(&json_str).expect("parse json");

    assert_eq!(json_val["version"], 1);
    assert_eq!(json_val["mediaFileName"], "stream.mkv");
    assert!(json_val["cutSegments"].is_array());
    assert_eq!(json_val["cutSegments"].as_array().unwrap().len(), 2);
    assert_eq!(json_val["cutSegments"][0]["start"], 5.0);
    assert_eq!(json_val["cutSegments"][0]["end"], 25.0);
    assert_eq!(json_val["cutSegments"][0]["label"], "Opening Hook");
}
