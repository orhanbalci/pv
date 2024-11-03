use polodb_core::Database;
use tdk_api::proverb_search;

pub mod proverb;
pub mod tdk_api;

fn main() {
    // let db = Database::open_path("pv.db").unwrap();
    // db.create_collection("proverbs").unwrap();
    download_proverbs();
}

fn download_proverbs() {
    let proverbs = proverb_search("a").unwrap();
    for p in proverbs {
        println!("{}", p.to_string());
    }
}
