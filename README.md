# audio-utils

### Usage

1. Compute stats for directory with wav files: `audio-utils stats <directory_path>`

```
# without logging

 ./target/release/audio-utils stats examples           
num threads: 10

Calucalting total duration for directory: "examples"
Number of wav files: 1
Total duration: 0.001 hours; 2.926 seconds
MIN duration: 2.926 sec
MAX duration: 2.926 sec
MEAN duration: 2.926 sec
MEDIAN duration: 2.926 sec

# with logging
RUST_LOG=info ./target/release/audio-utils stats examples
[2024-05-29T09:18:45Z INFO  audio_utils] num threads: 10

Calucalting total duration for directory: "examples"
Number of wav files: 1
Total duration: 0.001 hours; 2.926 seconds
MIN duration: 2.926 sec
MAX duration: 2.926 sec
MEAN duration: 2.926 sec
MEDIAN duration: 2.926 sec

[2024-05-29T09:18:45Z INFO  audio_utils] Executed in 655.416µs
```

2. Compute stats for NeMo style manifest: `audio-utils manifest <manifest_path>`

### TODO
- [x] add min/max/mean/median audio durations for both manifest and folder based stats
- [] Integrate [symphonia](https://github.com/pdeljanov/Symphonia) for corpus creation tools, such as conversion from mp3 to wav
- [] ONNX runtime to load onnx models in rust [onnxruntime-crate](https://docs.rs/onnxruntime/latest/onnxruntime/)