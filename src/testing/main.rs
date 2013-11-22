extern mod css_lex;
extern mod extra;
extern mod spec_css;
extern mod speculate;

use std::io::file_reader;
use std::cell::Cell;
use std::{task, os, from_str};
use css_lex::*;
use extra::time::precise_time_ns;
use speculate::SpecStats;

fn bench<T>(inner: &fn() -> T) -> (T, u64) {
    let ns_start = precise_time_ns();
    let res = inner();
    let ns_end = precise_time_ns();

    return (res, ns_end - ns_start);
}

fn seq_tokenize(path: Path) -> Result<((), u64), ()> {
    let p = Cell::new(path);
    do task::try {
        let reader = file_reader(&p.take()).unwrap();
        let css = reader.read_c_str();
        do bench {
            let mut a = tokenize(css);
            for _ in a {}
        }
    }
}

fn par_tokenize(path: Path) -> Result<(SpecStats, u64), ()> {
    let p = Cell::new(path);
    do task::try {
        let reader = file_reader(&p.take()).unwrap();
        let css = reader.read_c_str();
        let c = Cell::new(css);
        do bench {
            match spec_css::spec_tokenize(c.take(),
                                    from_str::from_str(os::args()[1]).unwrap_or(4)) {
                (s, _) => s
            }
        }
    }
}

fn main() {
    let base_dir = &Path("sample-data");
    let files = std::os::list_dir_path(base_dir);
    println!("name,seq,par,size,mispredicts");

    for file in files.iter() {
        let seq_time = seq_tokenize(file.clone());
        let par_time = par_tokenize(file.clone());
        match (seq_time, par_time) {
            (Ok((_, s)), Ok((p_stats, p_time))) =>
                println!("{},{:.4f},{:.4f},{},{}",
                         file.filename().unwrap(),
                         s as float / 1_000f,
                         p_time as float / 1_000f,
                         file.get_size().unwrap(),
                         p_stats.mispredictions.iter().count(|e| *e)),
            _ => (),
        }
    }
}
