use scilib::quantum::spherical_harmonics;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut coeffs: Vec<(usize, usize, f64, f64)> =
        load_coefficients("../01/mtmshc_AOHIS_20060101_00.360");

    coeffs = scale_coeffs(coeffs);

    println!("{:?}", coeffs[1]);
}

/// Scale the coefficients to the range [-1, 1]
fn scale_coeffs(mut coeffs: Vec<(usize, usize, f64, f64)>) -> Vec<(usize, usize, f64, f64)> {
    // Find the coefficients with the largest magnitude
    let max_c_nm: f64 = coeffs
        .iter()
        .map(|(_, _, c_nm, _)| c_nm.abs())
        .fold(0.0, |a, b| a.max(b));
    let max_s_nm: f64 = coeffs
        .iter()
        .map(|(_, _, _, s_nm)| s_nm.abs())
        .fold(0.0, |a, b| a.max(b));

    coeffs.iter_mut().for_each(|(_, _, c_nm, s_nm)| {
        *c_nm /= max_c_nm;
        *s_nm /= max_s_nm;
    });

    coeffs
}

/// Load spherical harmonic coefficients from a file
fn load_coefficients(file_path: &str) -> Vec<(usize, usize, f64, f64)> {
    let mut coefficients: Vec<(usize, usize, f64, f64)> = Vec::new();
    let file: File = File::open(file_path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut in_coefficients: bool = false;
    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");
        if line.trim() == "end_of_head" {
            in_coefficients = true;
            continue;
        }
        if !in_coefficients {
            continue;
        }
        if line.starts_with("gfc") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let n: usize = parts[1].parse::<usize>().unwrap();
            let m: usize = parts[2].parse::<usize>().unwrap();
            let c_nm: f64 = parts[3].replace("D", "E").parse::<f64>().unwrap();
            let s_nm: f64 = parts[4].replace("D", "E").parse::<f64>().unwrap();
            coefficients.push((n, m, c_nm, s_nm));
        }
    }
    coefficients
}
