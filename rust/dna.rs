// http://rosalind.info/problems/dna/
// Counting DNA Nucleotides

use std::fs;

fn main() {
    let target = "./target.txt";
    let dna = fs::read_to_string(target)
        .expect(&format!("error reading {}", target));

    let mut a: usize = 0;
    let mut c: usize = 0;
    let mut g: usize = 0;
    let mut t: usize = 0;
    
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            _ => (),
        }
    }

    println!("{} {} {} {}", a, c, g, t);
}
    
