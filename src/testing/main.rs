extern mod cssparser;
extern mod extra;
extern mod servo_style;

use std::io::file_reader;
use std::task;
use cssparser::*;
use extra::time::precise_time_ns;

fn bench(inner: &fn()) -> u64 {
    let ns_start = precise_time_ns();
    inner();
    let ns_end = precise_time_ns();

    return ns_end - ns_start;
}

fn bench_lex_one_file(path: &Path) -> Result<u64, ()> {
    let p = path.clone();
    do task::try {
        let my_path = &p.clone();
        let reader = file_reader(my_path).unwrap();
        let css = reader.read_c_str();
        do bench {
            let mut a = parse_stylesheet_rules(tokenize(css));
            for _ in a {}
        }
    }
}

fn bench_parse_one_file(path: &Path) -> Result<u64, ()> {
    // This is really redundant
    let p = path.clone();
    do task::try {
        let my_path = &p.clone();
        let reader = file_reader(my_path).unwrap();
        let css = reader.read_c_str();
        do bench {
            servo_style::stylesheets::parse_stylesheet(css);
        }
    }
}

fn main() {
    let base_dir = &Path("sample-data");
    let files = std::os::list_dir_path(base_dir);

    for file in files.iter() {
        let lex_time = bench_lex_one_file(file);
        let parse_time = bench_parse_one_file(file);
        match (lex_time, parse_time) {
            (Ok(l), Ok(p)) =>
                println(format!("{}:\tlex: {:.4f} ms\tparse: {:.4f} ms\tdiff: {:.4f} ms",
                                file.filename().unwrap(),
                                l as float / 1_000_000f,
                                p as float / 1_000_000f,
                                (p - l) as float / 1_000_000f)),
            _ => println(format!("{}: ERROR", file.filename().unwrap())),
        }
    }
}
