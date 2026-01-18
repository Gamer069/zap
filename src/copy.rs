use std::{io::{self, Write}, path::Path};
use yn::yes;
use rayon::prelude::*;

use crate::{Cli, path};

pub struct CopyJob {
    pub src: String,
    pub dest: String,
    pub overwrite: bool,
}

pub fn start(cli: Cli) {
    let src = cli.src;
    let dest = &cli.dest;
    let strict = cli.strict;
    let force = cli.force;

    if src.len() > 1 && let Err(err) = std::fs::create_dir_all(dest) {
        eprintln!("Failed to create destination directory {}: {}", dest, err);
        std::process::exit(-1);
    }

    let jobs = if force {
        src.into_par_iter().map(|src| {
            let dest = path::join_path_str(dest, path::filename_from_path(&src).unwrap());
            CopyJob { src, dest, overwrite: true }
        }).collect::<Vec<CopyJob>>().into_par_iter()
    } else {
        src.iter().map(|src| {
            let dest = path::join_path_str(dest, path::filename_from_path(src).unwrap());

            let overwrite = if path::path_exists_str(&dest) {
                let ask = ask(&format!("Override {}? [Y/n]", dest));
                yes(&ask) || ask == "\n".to_string()
            } else {
                true
            };
            CopyJob { src: src.clone(), dest, overwrite }
        }).collect::<Vec<CopyJob>>().into_par_iter()
    };

    jobs.for_each(|job| {
        copy(job, strict);
    });
}

pub fn copy(job: CopyJob, strict: bool) {
    if !job.overwrite { return; };

    let res = std::fs::copy(&job.src, &job.dest);

    if let Err(err) = res {
        eprintln!("Failed copying file {} to {}: {}", job.src, job.dest, err.to_string());

        if strict {
            std::process::exit(-1);
        }
    }
}

pub fn ask(msg: &str) -> String {
    let mut inp = String::new();

    print!("{}", msg);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    inp.trim_end().to_string()
}


pub fn parent(f: &str) -> String {
    let path = Path::new(f);

    match path.parent() {
        Some(parent) => parent.to_string_lossy().to_string(),
        None => root_path(),
    }
}

fn root_path() -> String {
    #[cfg(unix)]
    {
        "/".to_string()
    }

    #[cfg(windows)]
    {
        use std::env;
        let cwd = env::current_dir().expect("Cannot get current dir");
        cwd.components().next().unwrap().as_os_str().into()
    }
}
