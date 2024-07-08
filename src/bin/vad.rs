use ndarray::prelude::*;
use ndarray::{Array, ArrayD, Ix1, Ix2, IxDyn};
use ort::{GraphOptimizationLevel, Session, SessionInputs};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let ckpt_path = "/Users/tarassereda/rust_projects/audio-utils/src/ckpt/silero_vad.onnx";

    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file(ckpt_path)?;
    println!("{:?}", session);

    let meta = session.metadata()?;
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

    let frame = Array::<f32, Ix2>::zeros((1, 512));
    let sample_rate: ArrayBase<ndarray::OwnedRepr<i64>, Dim<[usize; 1]>> = array![16000];
    let mut state = ArrayD::<f32>::zeros([2, 1, 128].as_slice());

    let inputs = ort::inputs![
        frame,
        std::mem::take(&mut state),
        sample_rate
    ]?;
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
