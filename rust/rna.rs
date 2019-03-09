// http://rosalind.info/problems/rna/
// Transcribing DNA into RNA

use std::fs;

fn main() {
    let target = "./target.txt";
    let dna = fs::read_to_string(target)
        .expect(&format!("error reading {}", target));

    let mut rna = String::new();
    
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => rna.push('A'),
            'C' => rna.push('C'),
            'G' => rna.push('G'),
            'T' => rna.push('U'),
            _ => (), 
        };
    }

    println!("{}", rna);
}
    
