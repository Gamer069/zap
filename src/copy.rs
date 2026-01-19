use std::{io::{self, Write}, path::Path, time::Instant};
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
    let one = src.len() == 1;
    let time = cli.time;

    let now = Instant::now();

    if src.len() > 1 {
        if let Err(err) = std::fs::create_dir_all(dest) {
            eprintln!("Failed to create destination directory {}: {}", dest, err);
            std::process::exit(-1);
        }
    }

    let jobs = if force {
        src.into_par_iter().filter_map(|src| {
            let dest = if one {
                dest.to_string()
            } else {
                path::join_path_str(dest, path::filename_from_path(&src)?)
            };
            Some(CopyJob { src, dest, overwrite: true })
        }).collect::<Vec<CopyJob>>()
    } else {
        src.into_iter().filter_map(|src| {
            let dest = if one {
                dest.to_string()
            } else {
                path::join_path_str(dest, path::filename_from_path(&src)?)
            };

            let overwrite = if path::path_exists_str(&dest) {
                let ask = ask(&format!("Override {}? [Y/n]", dest));
                yes(&ask) || ask == "\n"
            } else {
                true
            };

            Some(CopyJob { src, dest, overwrite })
        }).collect::<Vec<CopyJob>>()
    };

    if time {
        println!("created jobs, {:#?}", now.elapsed());
    }

    jobs.into_par_iter().for_each(|job| {
        copy(job, strict);
    });

    if time {
        println!("copied files, {:#?}", now.elapsed());
    }
}

pub fn copy(job: CopyJob, strict: bool) {
    if !job.overwrite { return; };

    if let Err(err) = copy_impl(&job.src, &job.dest) {
        eprintln!("Failed copying file {} to {}: {}", job.src, job.dest, err);

        if strict {
            std::process::exit(-1);
        }
    }
}

pub fn copy_impl(src: &str, dest: &str) -> io::Result<()> {
    std::fs::copy(src, dest).map(|_| ())
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
