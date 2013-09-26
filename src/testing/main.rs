extern mod cssparser;
extern mod extra;

use std::io::file_reader;
use cssparser::*;
use extra::time::precise_time_ns;

fn print_res(res: Result<Rule,SyntaxError>) -> ~str {
    match res {
        Ok(rule) => match rule {
            QualifiedRule(qr) => qr.location.line.to_str(),
            AtRule(ar) => ar.location.line.to_str(),
        },
        Err(e) => e.to_str(),
    }
}

fn bench(inner: &fn()) -> u64 {
    let ns_start = precise_time_ns();
    inner();
    let ns_end = precise_time_ns();

    return ns_end - ns_start;
}

fn main() {
    let file = "sample-data/amazon11.css";
    let reader = file_reader(&Path(file)).unwrap();

    let b_time = do bench {
        let mut a = parse_stylesheet_rules(tokenize(reader.read_c_str()));
        for res in a {
            print_res(res);
        }
    };

    println(format!("time taken: {:u}μs", b_time / 1000));
}
