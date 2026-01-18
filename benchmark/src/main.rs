use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let num_files = 2000;
    let size_mb = 100;
    
    println!("Creating {} files of {}MB each...", num_files, size_mb);
    
    (1..=num_files).into_par_iter().for_each(|i| {
        let filename = format!("{}", i);
        let mut file = File::create(&filename).unwrap();
        
        // Create 100MB of data
        let chunk = vec![0u8; 1024 * 1024]; // 1MB chunks
        for _ in 0..size_mb {
            file.write_all(&chunk).unwrap();
        }
        
        if i % 100 == 0 {
            println!("Created {} files...", i);
        }
    });
    
    println!("Done!");
}
