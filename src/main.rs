use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let mut diff_buffer = 0.0f64;
    let mut i = 0f64;
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("result.txt")
        .expect("Faild to write in the file");
    while i < 1000000f64 {
        i += 1f64;

        let sqrt_num = i.sqrt();
        let sqrt_approx = sqrt_funktion(i);
        let diffrence = sqrt_approx - sqrt_num;

        diff_buffer += diffrence;
        writeln!(
            file,
            "{} --> {} ≅ {} = diff: {}",
            i, sqrt_num, sqrt_approx, diffrence
        )
        .expect("faild to wirte data");
    }

    writeln!(&mut file, "Average difference: {}", diff_buffer / i)
        .expect("Faild to ride in the file in the main");
    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);
}

fn sqrt_funktion(number: f64) -> f64 {
    let mut counter = 0f64;

    while number >= counter * counter {
        counter += 1f64;
    }

    counter -= 1f64;
    counter + (number - counter * counter) / (counter * 2f64)
}
