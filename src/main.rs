use std::fs::{File, OpenOptions};
use std::io::{self, Write};
fn main() {
    let mut diff_buffer = 0.0f64;
    let mut i = 0f64;

    while i < 150000000f64 {
        i += 1f64;

        diff_buffer += diff(sqrt_number(i), sqrt_funktion(i));
        file_input(
            i,
            sqrt_number(i),
            sqrt_funktion(i),
            diff(sqrt_number(i), sqrt_funktion(i)),
            &mut file(),
        )
        .expect("erro");
    }

    writeln!(&mut file(), "Average difference: {}", diff_buffer / i)
        .expect("Faild to ride in the file in the main");
}

fn sqrt_funktion(number: f64) -> f64 {
    let mut counter = 0f64;

    while number >= counter * counter {
        counter += 1f64;
    }

    counter -= 1f64;
    counter + (number - counter * counter) / (counter * 2f64)
}

fn sqrt_number(number: f64) -> f64 {
    number.sqrt()
}

fn diff(sqrt_number: f64, near_sqrt_number: f64) -> f64 {
    100f64 / sqrt_number * near_sqrt_number - 100f64
}

fn file() -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open("result.txt")
        .expect("Faild to write in the file")
}

fn file_input(
    number: f64,
    sqrt_number: f64,
    near_sqrt_number: f64,
    diff: f64,
    file: &mut File,
) -> io::Result<()> {
    match writeln!(
        file,
        "{} --> {} ≅ {} = diff{}%",
        number, sqrt_number, near_sqrt_number, diff
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Proplem by wirting in the file {}", e);
            Err(e)
        }
    }
}
