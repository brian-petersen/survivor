use rand::random;

const SAMPLE_SIZE: usize = 1_000_000;

fn perform_wave(sample: &mut Vec<bool>) {
    for e in &mut *sample {
        *e = random();
    }

    let survivors = sample.iter().filter(|e| **e).count();
    sample.resize(survivors, false);
}

fn main() {
    let mut sample = vec![false; SAMPLE_SIZE];

    println!("Trial\tStart\tDeaths");

    let mut trial = 1;
    while !sample.is_empty() {
        let start = sample.len();
        perform_wave(&mut sample);

        let death = start - sample.len();
        let ratio = death as f64 / start as f64;


        println!("{}\t{}\t{}\t{:.2}", trial, start, death, ratio);

        trial += 1;
    }
}
