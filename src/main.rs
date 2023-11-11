// Run: cargo run -- dir_path
//
// Executed in 101.834416ms
// Wow, my Rust total wav duration calculator is 300 times quicker than bellow approach that
// utilizes ffprobe and GNUparallel 
// ffprobe and GNUparallel: 
/*
 time find . -type f -name "*.wav" -print0 | parallel -0 ffprobe -hide_banner -v error -of default=noprint_wrappers=1:nokey=1 -show_entries stream=duration | paste - -sd+ - | bc
44294.400000
find . -type f -name "*.wav" -print0  0.00s user 0.01s system 0% cpu 17.198 total
parallel -0 ffprobe -hide_banner -v error -of  -show_entries stream=duration  142.36s user 88.11s system 785% cpu 29.345 total
paste - -sd+ -  0.01s user 0.03s system 0% cpu 29.345 total
bc  0.00s user 0.00s system 0% cpu 29.346 total
 */
use clap;
use glob::glob;
use hound;
use std::fs::File;
use std::time::{Duration, Instant};

fn wav_duration(f_name: &String) -> f32 {
    let f = File::open(f_name).unwrap();
    let reader = hound::WavReader::new(f).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let dur_sec = duration as f32 / spec.sample_rate as f32;

    //println!("duration: {:?} sec", dur_sec);
    return dur_sec;
}

fn wav_size(f_name: &String) {
    let mut f = File::open(f_name).unwrap();

    let size = hound::read_wave_header(&mut f).unwrap();
    println!("size: {:?}", size);
}
fn main() {
    let start = Instant::now();
    let dir_name = std::env::args().nth(1).expect("Failed to read argument");
    let glob_pattern = dir_name.clone() + "/**/*.wav";
    let mut total_dur = 0.0;
    let mut num_entires = 0;
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let dur = wav_duration(&path.into_os_string().into_string().unwrap());
                total_dur += dur;
                num_entires += 1;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Calucalting total duration for directory: {}", dir_name);
    println!("Number of wav files: {}", num_entires);
    println!("Total duration: {} hours", total_dur / 60.0 / 60.0);
    println!("Total duration: {} seconds", total_dur);
    let exec_duration = start.elapsed();
    println!("Executed in {:?}", exec_duration);
}
