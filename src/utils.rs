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

/// As pointed out in Jurafsky's SLP book: https://web.stanford.edu/~jurafsky/slp3/2.pdf
/// Levenstein also proposed a defintion of edit distance where substitutions are not allowed
/// This is equivaluent to setting sub_cust = 2, i.e. one subsitution now equals to 1 deletion and 1 insertion
pub fn edit_distance(seq_a: &str, seq_b: &str, sub_cost: Option<usize>) -> usize {
    

    let seq_a_vec: Vec<char> = seq_a.chars().collect();
    let seq_b_vec: Vec<char> = seq_b.chars().collect();

    let n_row = seq_a_vec.len() + 1;
    let n_col = seq_b_vec.len() + 1;

    let mut trellis = vec![vec![0; n_col]; n_row];
    for i in 0..n_row {
        trellis[i][0] = i;
    }
    for j in 0..n_col {
        trellis[0][j] = j;
    }
    for i in 1..n_row {
        for j in 1..n_col {

            let cost = if seq_a_vec[i-1] != seq_b_vec[j-1] {
                sub_cost.unwrap_or(1)
            } else {
                0
            };

            let min_dist = min(
                min(trellis[i][j-1] + 1, trellis[i-1][j] + 1), 
                trellis[i-1][j-1] + cost);

            trellis[i][j] = min_dist;
        }
    }

    trellis[n_row-1][n_col-1]
    

}
#[cfg(test)]
mod tests {
    use crate::utils::{wav_duration, wav_duration2, wav_size};
    use super::edit_distance;

    #[test]
    fn test_example() {
        let audio_path = "./examples/LJ001-0051.wav";
        let dur = wav_duration(audio_path);
        let dur2 = wav_duration2(audio_path);
        wav_size(audio_path);
        println!("duration {:?} {:?}", dur, dur2);
    }

    #[test]
    fn test_edit_distance() {
        
        let text_a = "intention";
        let text_b = "execution";
        let dist = edit_distance(text_a, text_b, Some(2));
        println!("Minimum edit distance {dist} for strings {text_a} and {text_b}");
    }
}