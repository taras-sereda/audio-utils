# audio-utils

### Usage

1. Compute stats for directory with wav files: `audio-utils stats <directory_path>`
2. Compute stats for NeMo style manifest: `audio-utils manifest <manifest_path>`

### TODO
- [] add min/max/mean/median audio durations for both manifest and folder based stats
- [] Integrate [symphonia](https://github.com/pdeljanov/Symphonia) for corpus creation tools, such as conversion from mp3 to wav
- [] ONNX runtime to load onnx models in rust [onnxruntime-crate](https://docs.rs/onnxruntime/latest/onnxruntime/)