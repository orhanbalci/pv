use std::{env, fmt::Debug, io};

use getopts::Options;
use polodb_core::{
    bson, options::UpdateOptions, Collection, CollectionT, Database, IndexModel, IndexOptions,
};
use proverb::Proverb;
use tdk_api::proverb_search;

pub mod proverb;
pub mod tdk_api;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("r", "refresh", "refresh proverb db from external service");
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

    let db = Database::open_path("pv.db").unwrap();
    if matches.opt_present("r") {
        let collection = init_db(&db);
        download_proverbs(&collection);
    } else if matches.opt_present("e") {
    } else {
        print_usage(&program, opts);
    }
}

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

pub fn init_db(db: &Database) -> Collection<Proverb> {
    let collections = db
        .list_collection_names()
        .expect("can not list collections");

    let collection;
    if !collections.contains(&String::from("proverbs")) {
        collection = db.collection::<Proverb>("proverbs");

        collection
            .create_index(IndexModel {
                keys: bson::doc! {
                  "id": 1,
                },
                options: Some(IndexOptions {
                    name: Some(String::from("id_1")),
                    unique: Some(true),
                }),
            })
            .expect("can not create index");
    } else {
        collection = db.collection::<Proverb>("proverbs");
    }
    collection
}

fn download_proverbs(collection: &Collection<Proverb>) {
    ('a'..='z').for_each(|c| {
        let proverbs = proverb_search(&c.to_string());
        match proverbs {
            Ok(p) => {
                println!("{} proverbs found for letter {}", p.len(), c);
                insert_proverb_bulk(collection, &p);
            }
            Err(e) => {
                println!("error while downloading proverbs for letter {}: {}", c, e);
            }
        }
    });
}

fn insert_proverb_bulk(collection: &Collection<Proverb>, p: &Vec<Proverb>) {
    let res = collection.insert_many(p);
    match res {
        Ok(_) => {}
        Err(_) => {
            println!("error while inserting proverbs");
        }
    };
}

fn insert_proverb(collection: &Collection<Proverb>, p: &Proverb) {
    collection
        .update_one_with_options(
            bson::doc! {
              "id": p.id,
            },
            bson::doc! {
              "$set": bson::doc! {
                "value": p.proverb.clone(),
                "meaning": p.meaning.clone(),
                "type": p.proverb_type.clone(),
              },
            },
            UpdateOptions { upsert: Some(true) },
        )
        .expect(&format!("can not insert proverb {}", p.proverb));
}
