use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let num_files = 1000000;
    let size_kb = 4;
    
    println!("Creating {} files of {}KB each...", num_files, size_kb);
    
    (1..=num_files).into_par_iter().for_each(|i| {
        let filename = format!("{}", i);
        let mut file = File::create(&filename).unwrap();
        
        let chunk = vec![0u8; 1000];
        for _ in 0..size_kb {
            file.write_all(&chunk).unwrap();
        }
        
        if i % 100 == 0 {
            println!("Created {} files...", i);
        }
    });
    
    println!("Done!");
}
