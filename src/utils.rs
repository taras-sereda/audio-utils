use hound::WavReader;
use std::{fs::File, path::Path};

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

#[cfg(test)]
mod tests {
    use crate::utils::{wav_duration, wav_duration2, wav_size};

    #[test]
    fn test_example() {
        let audio_path = "./examples/LJ001-0051.wav";
        let dur = wav_duration(audio_path);
        let dur2 = wav_duration2(audio_path);
        wav_size(audio_path);
        println!("duration {:?} {:?}", dur, dur2);
    }
}
