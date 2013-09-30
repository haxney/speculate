extern mod cssparser;
extern mod extra;
extern mod servo_style;

use std::io::file_reader;
use std::cell::Cell;
use std::task;
use cssparser::*;
use extra::time::precise_time_ns;

fn bench(inner: &fn()) -> u64 {
    let ns_start = precise_time_ns();
    inner();
    let ns_end = precise_time_ns();

    return ns_end - ns_start;
}

fn bench_lex_one_file(path: Path) -> Result<u64, ()> {
    let p = Cell::new(path);
    do task::try {
        let reader = file_reader(&p.take()).unwrap();
        let css = reader.read_c_str();
        do bench {
            let mut a = parse_stylesheet_rules(tokenize(css));
            for _ in a {}
        }
    }
}

fn bench_parse_one_file(path: Path) -> Result<u64, ()> {
    let p = Cell::new(path);
    do task::try {
        let reader = file_reader(&p.take()).unwrap();
        let css = reader.read_c_str();
        do bench {
            servo_style::stylesheets::parse_stylesheet(css);
        }
    }
}

fn main() {
    let base_dir = &Path("sample-data");
    let files = std::os::list_dir_path(base_dir);
    println!("name,lex,parse,size");

    for file in files.iter() {
        let lex_time = bench_lex_one_file(file.clone());
        let parse_time = bench_parse_one_file(file.clone());
        match (lex_time, parse_time) {
            (Ok(l), Ok(p)) =>
                println!("{},{:.4f},{:.4f},{}",
                         file.filename().unwrap(),
                         l as float / 1_000f,
                         p as float / 1_000f,
                         file.get_size().unwrap()),
            _ => (),
        }
    }
}
