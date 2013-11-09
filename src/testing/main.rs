extern mod css_lex;
extern mod extra;
extern mod spec_css;

use std::io::file_reader;
use std::cell::Cell;
use std::{task, os, from_str};
use css_lex::*;
use extra::time::precise_time_ns;

fn bench(inner: &fn()) -> u64 {
    let ns_start = precise_time_ns();
    inner();
    let ns_end = precise_time_ns();

    return ns_end - ns_start;
}

fn seq_tokenize(path: Path) -> Result<u64, ()> {
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

fn par_tokenize(path: Path) -> Result<u64, ()> {
    let p = Cell::new(path);
    do task::try {
        let reader = file_reader(&p.take()).unwrap();
        let css = reader.read_c_str();
        let c = Cell::new(css);
        do bench {
            spec_css::spec_tokenize(c.take(),
                                    from_str::from_str(os::args()[1]).unwrap_or(4));
        }
    }
}

fn main() {
    let base_dir = &Path("sample-data");
    let files = std::os::list_dir_path(base_dir);
    println!("name,seq,par,size");

    for file in files.iter() {
        let seq_time = seq_tokenize(file.clone());
        let par_time = par_tokenize(file.clone());
        match (seq_time, par_time) {
            (Ok(s), Ok(p)) =>
                println!("{},{:.4f},{:.4f},{}",
                         file.filename().unwrap(),
                         s as float / 1_000f,
                         p as float / 1_000f,
                         file.get_size().unwrap()),
            _ => (),
        }
    }
}
