// http://rosalind.info/problems/recv/
// Complementing a Strand of DNA

use std::fs;

fn main() {
    let target = "./target.txt";
    let dna = fs::read_to_string(target)
        .expect(&format!("error reading {}", target));

    let mut rna = String::new();
    
    for nucleotide in dna.chars() {
        match nucleotide {
            // A <=> T
            // C <=> G
            'A' => rna.push('T'),
            'C' => rna.push('G'),
            'G' => rna.push('C'),
            'T' => rna.push('A'),
            _ => (), 
        };
    }

    println!("{}", rna.chars().rev().collect::<String>());
}
    
