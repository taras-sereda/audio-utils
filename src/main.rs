// Run with options to output manifest
// ./target/release/audio-utils -w ~/data/poly/audios  res/man.jsonl
//
// Rayon
// Run: cargo run --release -- dir_path
// Executed in 80.586541ms
//
// Baseline
// Run: cargo run --release -- dir_path
// Executed in 89.358584ms
//
// Baseline(debug)
// Run: cargo run -- dir_path
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

use clap::{Parser, Subcommand};
use glob::glob;
use log::{debug, info};
use rayon::current_num_threads;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use std::{fs::File, path::Path};

mod utils;

use crate::utils::{edit_distance, wav_duration2};

#[derive(Parser)]
// #[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compute audio stats. At the moment the following stats are computed: number of wav files; total duration of all wav files.
    Stats {
        ///input directory with *.wav files
        #[arg(default_value = "./")]
        path: std::path::PathBuf,

        ///write manifest to jsonl or not
        #[arg(short, long)]
        write_manifest: bool,

        ///otput manifest path
        #[arg(default_value = "manifest.jsonl")]
        output_path: String,
    },
    /// Read audio stats from provided manifest file.
    Manifest {
        path: std::path::PathBuf,
    },
    Dist {
        text_a: String,
        text_b: String,
    },
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Ok(BufReader::new(file).lines())
}

#[derive(Serialize, Deserialize)]
struct Datapoint {
    audio_filepath: String,
    text: String,
    duration: f32,
}

fn compute_and_print_stats(path: &PathBuf, durations: Vec<f32>) {
    let total_dur: f32 = durations.iter().sum();
    let num_entires = durations.len();
    let mut sorted_durations = durations.clone();
    sorted_durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min_dur = sorted_durations[0];
    let max_dur = sorted_durations[num_entires - 1];
    let mean_dur = total_dur / num_entires as f32;
    let median_dur = sorted_durations[num_entires / 2];

    println!();
    println!("Calucalting total duration for directory: {:?}", path);
    println!("Number of wav files: {}", num_entires);
    println!(
        "Total duration: {:.3} hours; {:.3} seconds",
        total_dur / 60.0 / 60.0,
        total_dur
    );
    println!("MIN duration: {:.3} sec", min_dur);
    println!("MAX duration: {:.3} sec", max_dur);
    println!("MEAN duration: {:.3} sec", mean_dur);
    println!("MEDIAN duration: {:.3} sec", median_dur);
    println!();
}
fn main() -> Result<(), glob::PatternError> {
    env_logger::init();
    let start = Instant::now();
    let args = Cli::parse();
    let global_num_threads = current_num_threads();
    info!("num threads: {}", global_num_threads);

    match &args.command {
        Command::Stats {
            path,
            write_manifest,
            output_path,
        } => {
            let glob_pattern = path.clone().into_os_string().into_string().unwrap() + "/**/*.wav";
            let entries: Vec<_> = glob(&glob_pattern)?.filter_map(|path| path.ok()).collect();
            let durations: Vec<_> = entries.par_iter().map(|path| wav_duration2(path)).collect();

            if *write_manifest {
                let f_desc = File::create(output_path).expect("something went wrong");
                let mut writer = BufWriter::new(f_desc);
                let iter = std::iter::zip(entries, &durations);
                for elem in iter {
                    let json_value = serde_json::json!({
                        "audio_filepath": elem.0.file_name().unwrap().to_str(),
                        "duration": elem.1 
                    });
                    serde_json::to_writer(&mut writer, &json_value).unwrap();
                    write!(writer, "\n").expect("failed to write");
                }
                writer.flush().unwrap();
            }

            let exec_duration = start.elapsed();
            compute_and_print_stats(path, durations);
            info!("Executed in {:?}", exec_duration);
        }
        Command::Manifest { path } => {
            let lines = read_lines(path).unwrap();
            let mut durations: Vec<f32> = Vec::new();
            for line in lines {
                let datapoint: Datapoint = serde_json::from_str(line.unwrap().as_str()).unwrap();
                durations.push(datapoint.duration);
            }

            let exec_duration = start.elapsed();
            compute_and_print_stats(path, durations);
            info!("Executed in {:?}", exec_duration);
        }
        Command::Dist { text_a, text_b } => {
            let dist = edit_distance(&text_a, &text_b, Some(1));
            let exec_duration = start.elapsed();
            println!();
            println!("text_a {text_a}");
            println!("text_b {text_b}");
            println!("Edit distance: {dist}");
            println!();
            info!("Executed in {:?}", exec_duration);
        }
    }

    Ok(())
}
