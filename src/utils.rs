use hound::WavReader;
use std::{fs::File, path::Path};
use std::cmp::min;


pub fn wav_duration2<AR>(f_name: AR) -> f32
where
    AR: AsRef<Path>,
{
    //let sleep_dur = time::Duration::from_millis(1000);
    //thread::sleep(sleep_dur);
    //let th_idx = current_thread_index().unwrap();
    //println!("{}", th_idx);
    let f = File::open(f_name.as_ref()).unwrap();
    let reader = WavReader::new(f).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let dur_sec = duration as f32 / spec.sample_rate as f32;

    dur_sec
}

pub fn wav_duration(f_name: &str) -> f32 {
    let f = File::open(f_name).unwrap();
    let reader = WavReader::new(f).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let dur_sec = duration as f32 / spec.sample_rate as f32;

    dur_sec
}

pub fn wav_size(f_name: &str) {
    let mut f = File::open(f_name).unwrap();

    let size = hound::read_wave_header(&mut f).unwrap();
    println!("size: {:?} bytes", size);
}

pub fn edit_distance(seq_a: &str, seq_b: &str) -> usize {

    let n_row = seq_a.chars().count() + 1;
    let n_col = seq_b.chars().count() + 1;

    let mut trellis = vec![vec![0; n_col]; n_row];
    for i in 0..n_row {
        trellis[i][0] = i;
    }
    for j in 0..n_col {
        trellis[0][j] = j;
    }

    for (i, char_a) in seq_a.char_indices() {
        for (j, char_b) in seq_b.char_indices() {
            let cost = (char_a != char_b) as usize;
            let dist = min(min(trellis[i+1][j], trellis[i][j+1]), trellis[i][j]);
            trellis[i+1][j+1] = dist + cost;
        }
    }

    trellis[n_row-1][n_col-1]
    
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::utils::{wav_duration, wav_duration2, wav_size};
    use super::edit_distance;

    #[test]
    fn test_example() {
        let audio_path = "/Users/vickost/data/im-data/audios-resampled/01-09-2023 19-29-24.039363-0-125447265.625.wav";
        let dur = wav_duration(audio_path);
        let dur2 = wav_duration2(audio_path);
        wav_size(audio_path);
        println!("duration {:?} {:?}", dur, dur2);
    }

    #[test]
    fn test_edit_distance() {
        
        // let text_1 = "While these aspirations can be at odds with fast build times and low binary size, we will still strive to keep these reasonable for the flexibility you get. Check out the argparse-benchmarks for CLI parsers optimized for other use cases.";
        // let text_2 = "While these aspirations can be at odds with fast build times and low binary size, we will still strive to keep these reasonable for the flexibility you get. Check out the argparse-benchmarks for CLI parsers optimized for other use cases.";
        // let start = Instant::now();
        // let dist_1 = edit_distance(text_1, text_1);
        // println!("Minimum edit distance {dist_1} for strings {text_1} and {text_2}");
        // let duration = start.elapsed();
        // println!("Execution took {duration:?}");

        let text_3 = "intention";
        let text_4 = "execution";
        let dist_2 = edit_distance(text_3, text_4);
        println!("Minimum edit distance {dist_2} for strings {text_3} and {text_4}");
    }
}