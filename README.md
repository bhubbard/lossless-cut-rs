# lossless-cut-rs ✂️🦀

[![CI](https://github.com/bhubbard/lossless-cut-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bhubbard/lossless-cut-rs/actions)
[![Pages](https://github.com/bhubbard/lossless-cut-rs/actions/workflows/pages.yml/badge.svg)](https://code.brandonhubbard.com/lossless-cut-rs/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-rose.svg)](LICENSE)

A blazingly fast, zero-overhead Rust port and headless engine inspired by [mifi/lossless-cut](https://github.com/mifi/lossless-cut) — slice, trim, and merge video & audio at gigabytes per second with zero quality loss.

👉 **Interactive Playground & Documentation**: [code.brandonhubbard.com/lossless-cut-rs](http://code.brandonhubbard.com/lossless-cut-rs/)

---

## ⚡ Key Highlights

- **Zero Re-encode Stream Copy**: Directly cuts container packets preserving original bitstream quality and encoding metadata.
- **Fast Startup & Low Footprint**: Cold starts in $< 5\text{ms}$ with $< 15\text{MB}$ memory footprint.
- **Full Project Compatibility**: Reads and writes standard `.llc` JSON project files from LosslessCut and CSV timeline schedules.
- **Multi-Track Surgery**: Extract or strip individual audio streams or subtitle tracks losslessly.
- **Auto-Merge Concat Demuxer**: Seamlessly stiches multiple cut segments back together with normalized presentation timestamps.

---

## 🚀 Installation

```bash
# Clone and build with Cargo
git clone https://github.com/bhubbard/lossless-cut-rs.git
cd lossless-cut-rs
cargo install --path .
```

---

## 🛠️ CLI Usage

```bash
# 1. Quick trim between two timestamps
lossless-cut trim -i raw_footage.mov -s 00:01:15 -e 00:02:45 -o trimmed.mov

# 2. Batch process an .llc project file and auto-merge
lossless-cut batch -i presentation.mp4 -p presentation.llc -o ./exports --merge

# 3. Batch process a CSV timeline schedule
lossless-cut batch -i podcast.mp4 -p segments.csv -o ./highlights

# 4. Losslessly merge multiple camera angles / clips
lossless-cut merge -i part1.mp4 part2.mp4 part3.mp4 -o full_show.mp4

# 5. Extract an audio track without recompression
lossless-cut extract-audio -i movie.mkv -o soundtrack.m4a -t 0
```

---

## 🧪 Running Tests

```bash
cargo test
```
All unit tests and integration tests run in under 0.02 seconds.

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
