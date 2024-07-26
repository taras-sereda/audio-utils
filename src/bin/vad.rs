// USAGE: cargo run --bin vad <audio_path>

use ndarray::prelude::*;
use ndarray::{Array, ArrayD, Ix2};
use ort::{GraphOptimizationLevel, Session};
use std::env;
use std::error::Error;

fn get_model_details(session: &Session) {
    println!("ONNX runtime model details are as follows:");

    let meta = session.metadata().unwrap();
    if let Ok(x) = meta.name() {
        println!("Name: {}", x);
    }
    if let Ok(x) = meta.description() {
        println!("Description: {}", x);
    }
    if let Ok(x) = meta.producer() {
        println!("Producer: {}", x);
    }
    println!("Inputs:");
    for (idx, input) in session.inputs.iter().enumerate() {
        println!("    {idx} {} {:?}", input.name, input.input_type);
    }
    println!("Outputs:");

    for (idx, output) in session.outputs.iter().enumerate() {
        println!("    {idx} {} {:?}", output.name, output.output_type);
    }
}

#[test]
fn test_iference() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let ckpt_path = current_dir.join("src/ckpt/silero_vad.onnx");

    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file(ckpt_path)?;

    let frame = Array::<f32, Ix2>::zeros((1, 512));
    let sample_rate: ArrayBase<ndarray::OwnedRepr<i64>, Dim<[usize; 1]>> = array![16000];
    let mut state = ArrayD::<f32>::zeros([2, 1, 128].as_slice());

    let inputs = ort::inputs![frame, std::mem::take(&mut state), sample_rate]?;
    let res = session.run(ort::SessionInputs::ValueSlice::<3>(&inputs))?;
    let (out_shape, out_data) = res["output"].try_extract_raw_tensor::<f32>()?;
    println!("{:?}", out_shape);
    println!("{:?}", *out_data.first().unwrap());
    // let state1 = Array::from_vec(res["stateN"].try_extract_raw_tensor().unwrap().1.to_owned());
    let new_state = res["stateN"].try_extract_raw_tensor::<f32>()?.1;
    // println!("{:?}", state);
    // println!("{:?}", new_state);
    // println!("state shape: {:?}", state.shape());
    state = ArrayD::from_shape_vec([2, 1, 128].as_slice(), new_state.to_vec())?;
    // state = ArrayD::<f32>::zeros([2, 1, 128].as_slice());
    // println!("Populating the state");
    // println!("{:?}", state);
    // println!("{:?}", state3);
    // println!(
    //     "{:?}",
    //     *res["output"]
    //         .try_extract_raw_tensor::<f32>()
    //         .unwrap()
    //         .1
    //         .first()
    //         .unwrap()
    // );
    // println!("YAY!");
    // let inputs = ort::inputs![
    //     frame.clone(),
    //     std::mem::take(&mut state),
    //     sample_rate.clone()
    // ]?;
    // let res = session.run(ort::SessionInputs::ValueSlice::<3>(&inputs))?;
    // let (out_shape, out_data) = res["output"].try_extract_raw_tensor::<f32>()?;
    // println!("{:?}", out_shape);
    // println!("{:?}", *out_data.first().unwrap());
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let ckpt_path = current_dir.join("src/ckpt/silero_vad.onnx");

    let audio_path = std::env::args().nth(1).unwrap_or_else(|| {
        current_dir
            .join("examples/LJ001-0051.wav")
            .into_os_string()
            .into_string()
            .unwrap()
    });
    let mut wav_reader = hound::WavReader::open(audio_path).unwrap();
    let sample_rate = match wav_reader.spec().sample_rate {
        8000 => audio_utils::utils::SampleRate::EightkHz,
        16000 => audio_utils::utils::SampleRate::SixteenkHz,
        _ => panic!("Unsupported sample rate. Expect 8 kHz or 16 kHz."),
    };
    if wav_reader.spec().sample_format != hound::SampleFormat::Int {
        panic!("Unsupported sample format. Expect Int.");
    }
    let content = wav_reader
        .samples()
        .filter_map(|x| x.ok())
        .collect::<Vec<i16>>();
    assert!(!content.is_empty());
    let silero = audio_utils::silero::Silero::new(sample_rate, ckpt_path.clone()).unwrap();
    let vad_params = audio_utils::utils::VadParams {
        sample_rate: sample_rate.into(),
        ..Default::default()
    };
    let mut vad_iterator = audio_utils::vad_iter::VadIter::new(silero, vad_params);
    vad_iterator.process(&content).unwrap();
    for timestamp in vad_iterator.speeches() {
        let speech_start = timestamp.start as f32 / 16000.0;
        let speech_end = timestamp.end as f32 / 16000.0;

        println!(
            "Speech start:{} sec; end: {} sec.",
            speech_start, speech_end
        );
    }
    println!("Finished.");

    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file(ckpt_path)?;

    get_model_details(&session);
    print!("Is speech? {}", vad_iterator.is_speech(&content)?);
    Ok(())
}
