# TODO: `lossless-cut-rs` ✂️🦀

A blazingly fast, zero-overhead Rust port and headless engine inspired by [mifi/lossless-cut](https://github.com/mifi/lossless-cut) — the swiss army knife of lossless video, audio, and subtitle surgery.

---

## 🎯 Mission & Goals

- **Zero-Re-encode Lossless Trimming**: Cut, slice, and rearrange media segments in milliseconds with zero quality loss and no transcoding overhead.
- **Smart Cut Engine**: Perform frame-accurate cuts by re-encoding *only* the few non-keyframe frames at the cut boundary (the lead-in/lead-out GOP), while stream-copying the bulk of the footage untouched.
- **Track & Stream Surgery**: Non-destructively add, strip, reorder, or extract audio tracks, subtitles, and chapter markers without remux penalties.
- **Full Project & Timeline Compatibility**: Read/write `.llc` JSON project files, EDL, CSV, and Final Cut Pro XML formats.

---

## 🏗️ Crates Architecture Plan

- [ ] `lossless-cut-core`: Segment model (`CutSegment`, `StreamSpec`, `MediaType`), project serialization, and timeline math.
- [ ] `lossless-cut-demux`: Micro-second packet indexing, keyframe/IDR packet locator, and GOP analyzer.
- [ ] `lossless-cut-smartcut`: Targeted frame re-encoder for smart frame-accurate boundaries.
- [ ] `lossless-cut-mux`: Fast container slicer/concatenator for MP4, MKV, WebM, MOV, and TS.
- [ ] `lossless-cut-cli`: High-throughput CLI and headless JSON-RPC/IPC daemon for external GUI and script automation.

---

## 📋 Implementation Checklist

### Phase 1: Core Data Models & Keyframe Indexing
- [ ] Define core segment and track models:
  ```rust
  #[derive(Debug, Serialize, Deserialize)]
  pub struct CutSegment {
      pub start: RationalTime,
      pub end: RationalTime,
      pub name: Option<String>,
      pub tags: HashMap<String, String>,
  }
  ```
- [ ] Rapid packet-level keyframe indexer (scans packet headers for `AV_PKT_FLAG_KEY` / IDR without decoding raw frames).
- [ ] Accurate calculation of nearest previous and next keyframe PTS/DTS timestamps.

### Phase 2: Lossless Slicing & Merging
- [ ] Packet-level slicing preserving packet presentation timestamps (`pts`) and decode timestamps (`dts`).
- [ ] Stream copy concatenator: seamlessly stitch together multiple segments from the same source file into a single output file.
- [ ] Audio/video sync drift prevention: timestamp normalization across segment joints.

### Phase 3: "Smart Cut" (Frame-Accurate Cutting)
- [ ] Detect if cut point aligns exactly on a keyframe (Keyframe Cut) or within a GOP (Smart Cut needed).
- [ ] For non-keyframe cuts:
  - [ ] Decode frames from cut `start` to the next keyframe.
  - [ ] Re-encode only that mini-segment matching source codec parameters, profile, color primaries, and SAR/DAR.
  - [ ] Stream-copy the remaining keyframe-aligned segments.
  - [ ] Re-encode the trailing tail segment to the cut `end`.
  - [ ] Losslessly concatenate [Head Re-encode] + [Stream Copy Body] + [Tail Re-encode].

### Phase 4: Track Manipulation & Metadata
- [ ] Audio track management: drop specific audio streams, extract audio to standalone files (AAC, MP3, Opus).
- [ ] Subtitle track extraction (SRT, VTT, ASS) and insertion.
- [ ] Rotation tag editor: change display rotation metadata (0°, 90°, 180°, 270°) instantly without re-encoding.
- [ ] Chapter marker injection and extraction.

### Phase 5: Project Format & Interchange
- [ ] Parse and generate `.llc` JSON project files (100% compatible with desktop LosslessCut).
- [ ] CMX 3600 EDL (Edit Decision List) import and export.
- [ ] CSV / TSV segment export.
- [ ] Final Cut Pro 7 XML and FCPXML timeline export.

### Phase 6: Headless CLI & Automation
- [ ] Fast CLI commands:
  ```bash
  lossless-cut-rs cut input.mp4 --segments "00:01:10-00:02:15,00:05:00-00:06:30" --merge -o out.mp4
  lossless-cut-rs smart-cut input.mp4 --start 00:01:12.450 --end 00:03:15.200 -o out.mp4
  ```
- [ ] Headless daemon / HTTP API mode for integration into web apps and video pipelines.

### Phase 7: Benchmarks & Parity Tests
- [ ] Benchmark cut execution time vs standard FFmpeg CLI and LosslessCut Electron backend.
- [ ] Frame-accuracy validation suite across H.264, H.265, VP9, and AV1 sample files.
