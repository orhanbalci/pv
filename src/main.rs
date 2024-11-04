use std::{env, io};

use getopts::Options;
use polodb_core::Database;
use tdk_api::proverb_search;

pub mod proverb;
pub mod tdk_api;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "r",
        "refresh",
        "refresh proverb db from external service",
        "",
    );
    opts.optopt("e", "export", "export proverbs info to json", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            panic!("failed to read program arguments")
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("r") {
    } else if matches.opt_present("e") {
    } else {
        print_usage(&program, opts);
    }

    // let db = Database::open_path("pv.db").unwrap();
    // db.create_collection("proverbs").unwrap();
    // download_proverbs();
}

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn download_proverbs() {
    let proverbs = proverb_search("a").unwrap();
    for p in proverbs {
        println!("{}", p.to_string());
    }
}
