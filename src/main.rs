use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{:?}",
        load_coefficients("../01/mtmshc_AOHIS_20060101_00.360")
    );
}

/// Load spherical harmonic coefficients from a file
fn load_coefficients(file_path: &str) -> Vec<(usize, usize, f64, f64)> {
    let mut coefficients = Vec::new();
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut in_coefficients = false;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim() == "end_of_head" {
            in_coefficients = true;
            continue;
        }
        if !in_coefficients {
            continue;
        }
        if line.starts_with("gfc") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let n = parts[1].parse::<usize>().unwrap();
            let m = parts[2].parse::<usize>().unwrap();
            let c_nm = parts[3].replace("D", "E").parse::<f64>().unwrap();
            let s_nm = parts[4].replace("D", "E").parse::<f64>().unwrap();
            coefficients.push((n, m, c_nm, s_nm));
        }
    }
    coefficients
}
