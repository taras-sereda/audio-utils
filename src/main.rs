// Run: cargo run -- dir_path
//
// Executed in 101.834416ms
// Wow, my Rust total wav duration calculator is 300 times quicker than bellow approach that
// utilizes ffprobe and GNUparallel:
/*
 time find . -type f -name "*.wav" -print0 | parallel -0 ffprobe -hide_banner -v error -of default=noprint_wrappers=1:nokey=1 -show_entries stream=duration | paste - -sd+ - | bc
44294.400000
find . -type f -name "*.wav" -print0  0.00s user 0.01s system 0% cpu 17.198 total
parallel -0 ffprobe -hide_banner -v error -of  -show_entries stream=duration  142.36s user 88.11s system 785% cpu 29.345 total
paste - -sd+ -  0.01s user 0.03s system 0% cpu 29.345 total
bc  0.00s user 0.00s system 0% cpu 29.346 total
 */

use clap::Parser;
use glob::glob;
use hound;
use rayon::prelude::*;
use rayon::{current_num_threads, current_thread_index};
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{thread, time};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}
fn wav_duration(f_name: &String) -> f32 {
    let f = File::open(f_name).unwrap();
    let reader = hound::WavReader::new(f).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let dur_sec = duration as f32 / spec.sample_rate as f32;

    dur_sec
}

fn wav_duration2<AR>(f_name: AR) -> f32
where
    AR: AsRef<Path>,
{
    //let sleep_dur = time::Duration::from_millis(1000);
    //thread::sleep(sleep_dur);
    let th_idx = current_thread_index().unwrap();
    println!("{}", th_idx);
    let f = File::open(f_name.as_ref()).unwrap();
    let reader = hound::WavReader::new(f).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let dur_sec = duration as f32 / spec.sample_rate as f32;

    dur_sec
}

fn wav_size(f_name: &String) {
    let mut f = File::open(f_name).unwrap();

    let size = hound::read_wave_header(&mut f).unwrap();
    println!("size: {:?}", size);
}
fn main() -> Result<(), glob::PatternError> {
    let start = Instant::now();
    let args = Cli::parse();
    let global_num_threads = current_num_threads();
    println!("num threads: {}", global_num_threads);

    let glob_pattern = args.path.clone().into_os_string().into_string().unwrap() + "/**/*.wav";
    let entries: Vec<_> = glob(&glob_pattern)?.filter_map(|path| path.ok()).collect();
    let durations: Vec<_> = entries.par_iter().map(|path| wav_duration2(path)).collect();
    let total_dur: f32 = durations.iter().sum();
    let num_entires = durations.len();

    println!("Calucalting total duration for directory: {:?}", args.path);
    println!("Number of wav files: {}", num_entires);
    println!("Total duration: {} hours", total_dur / 60.0 / 60.0);
    println!("Total duration: {} seconds", total_dur);
    let exec_duration = start.elapsed();
    println!("Executed in {:?}", exec_duration);
    Ok(())
}
