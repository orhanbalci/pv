use std::io::Write;
use std::{collections::HashMap, env, fs::File};

use anyhow::{Context, Result};
use asky::Select;
use emojic::flat::*;
use getopts::Options;
use polodb_core::{
    bson::doc, options::UpdateOptions, Collection, CollectionT, Database, IndexModel, IndexOptions,
};
use proverb::Proverb;
use quiz::{Question, Quiz};
use rand::{seq::SliceRandom, Rng};
use regex::Regex;
use serde_json::to_writer_pretty;
use tdk_api::proverb_search;

pub mod proverb;
pub mod quiz;
pub mod tdk_api;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag(
        "g",
        "guncelle",
        "kayitli deyim/atasozlerini tdk sozlugunden guncelle",
    );
    opts.optflag("s", "sayi", "veritabaninda kayitli deyim/atasozu sayisi");
    opts.optopt("c", "cikti", "deyim/atasozlerini diske kaydet", "DOSYA");
    opts.optopt(
        "f",
        "format",
        "deyim/atasozlerini kaydetme formati (json, sql)",
        "FORMAT",
    );
    opts.optflag(
        "q",
        "quiz",
        "kayitli deyim/atasozlerinden 10 soruluk quiz olustur",
    );
    opts.optflag("h", "yardim", "yardim menusunu goster");

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
    if matches.opt_present("g") {
        refresh_proverb_db(db);
    } else if matches.opt_present("c") {
        if matches.opt_present("f") {
            handle_export(
                &db,
                matches.opt_str("c").unwrap(),
                matches.opt_str("f").unwrap(),
            );
        } else {
            print_usage(&program, opts);
        }
    } else if matches.opt_present("s") {
        handle_proverb_count(&db);
    } else if matches.opt_present("q") {
        handle_quiz(&db);
    } else {
        print_usage(&program, opts);
    }
}

fn handle_quiz(db: &Database) {
    let mut quiz = prepare_quiz(db);
    quiz.questions.iter_mut().for_each(|q| {
        let choice = Select::new(&q.proverb, q.options.clone()).prompt();

        match choice {
            Ok(selection) => {
                q.user_answer = selection;
                if q.is_correct() {
                    println!("{} Dogru!", CHECK_MARK_BUTTON);
                } else {
                    println!("{} Yanlis!", CROSS_MARK);
                }
            }
            Err(_) => println!("Hata olustu"),
        }

        println!("");
    });

    quiz.print_score();
}

fn handle_proverb_count(db: &Database) {
    let proverb_count = db
        .collection::<Proverb>("proverbs")
        .count_documents()
        .context("can not count proverbs");
    match proverb_count {
        Ok(c) => {
            println!("{} proverbs found in db", c);
        }
        Err(e) => {
            println!("error while counting proverbs: {}", e);
        }
    }
}

fn refresh_proverb_db(db: Database) {
    let collection = init_db(&db);
    download_proverbs(&collection);
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
                keys: doc! {
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
    let mut proverb_set = HashMap::new();

    ('a'..='z').for_each(|c| {
        let proverbs = proverb_search(&c.to_string());
        match proverbs {
            Ok(p) => {
                p.iter().for_each(|p| {
                    if !proverb_set.contains_key(&p.id) {
                        proverb_set.insert(p.id, p.clone());
                    }
                });
                println!("{} proverbs found for letter {}", p.len(), c);
                // insert_proverb_bulk(collection, &p);
                // p.iter().for_each(|p| insert_proverb(collection, p));
            }
            Err(e) => {
                println!("error while downloading proverbs for letter {}: {}", c, e);
            }
        }
    });

    insert_proverb_bulk(collection, &proverb_set.values().cloned().collect());
}

fn insert_proverb_bulk(collection: &Collection<Proverb>, p: &Vec<Proverb>) {
    let res = collection.insert_many(p);
    match res {
        Err(e) => {
            println!("error while bulk inserting proverbs {}", e);
        }
        _ => {}
    };
}

fn _list_proverbs_with_id(db: &Database) {
    let collection = db.collection::<Proverb>("proverbs");
    let proverbs_with_id_1 = collection
        .find(doc! {
            "id": { "$eq": 1941 },
        })
        .run();
    match proverbs_with_id_1 {
        Ok(p) => {
            for proverb in p {
                println!("{:?}", proverb.unwrap());
            }
        }
        Err(e) => {
            println!("error while listing proverbs: {}", e);
        }
    }
}

fn _list_first_100_proverbs(db: &Database) {
    let mut proverbs: Vec<Proverb> = db
        .collection::<Proverb>("proverbs")
        .find(doc! {
            "id": { "$lt": 100 },
        })
        .limit(100)
        .run()
        .expect("can not list proverbs")
        .map(|p| p.unwrap())
        .collect();

    proverbs.sort_by(|a, b| a.id.cmp(&b.id));

    proverbs.iter().for_each(|p| {
        println!("{:?}", p);
    });
}

fn handle_export(db: &Database, filename: String, format: String) {
    // retrieve all proverbs, sort by id
    let mut proverbs: Vec<Proverb> = db
        .collection::<Proverb>("proverbs")
        .find(doc! {})
        .run()
        .expect("can not list proverbs")
        .map(|p| p.unwrap())
        .collect();

    proverbs.sort_by(|a, b| a.id.cmp(&b.id));

    if format == "json" {
        let export_result = export_to_json(&proverbs, filename.as_str());
        match export_result {
            Ok(_) => {
                println!("proverbs exported to {}", filename);
            }
            Err(e) => {
                println!("error while exporting proverbs: {}", e);
            }
        }
    } else if format == "sql" {
        let export_result = export_to_sql(&proverbs, filename.as_str());
        match export_result {
            Ok(_) => {
                println!("proverbs exported to {}", filename);
            }
            Err(e) => {
                println!("error while exporting proverbs: {}", e);
            }
        }
    } else {
        println!("unknown format: {}", format);
    }
}

fn export_to_sql(proverbs: &[Proverb], filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(
        file,
        "CREATE TABLE proverb (
            id    integer PRIMARY KEY,
            proverb   varchar(500) NOT NULL,
            meaning   varchar(500) NOT NULL,
            proverb_type   varchar(10) NOT NULL,
         );"
    )?;

    proverbs.iter().for_each(|p| {
        _ = writeln!(
            file,
            "INSERT INTO proverb (id, proverb, meaning, proverb_type) VALUES ({}, '{}', '{}', '{}');",
            p.id, p.proverb, p.meaning, p.proverb_type
        );
    });

    Ok(())
}

fn export_to_json(items: &Vec<Proverb>, filename: &str) -> Result<()> {
    // Open or create the file
    let file = File::create(filename)?;

    // Serialize the Vec to JSON and write to the file with pretty formatting
    to_writer_pretty(file, &items)?;

    Ok(())
}

fn _insert_proverb(collection: &Collection<Proverb>, p: &Proverb) {
    collection
        .update_one_with_options(
            doc! {
              "id": p.id,
            },
            doc! {
              "$set":doc! {
                "proverb": p.proverb.clone(),
                "meaning": p.meaning.clone(),
                "proverb_type": p.proverb_type.clone(),
              },
            },
            UpdateOptions { upsert: Some(true) },
        )
        .expect(&format!("can not insert proverb {}", p.proverb));
}

fn prepare_quiz(db: &Database) -> Quiz {
    let proverbs: Vec<Proverb> = db
        .collection::<Proverb>("proverbs")
        .find(doc! {})
        .run()
        .expect("can not list proverbs")
        .map(|p| p.unwrap())
        .collect();

    let mut rng = rand::thread_rng();
    let mut quiz = Quiz::new();

    for _ in 0..10 {
        let random_index = rng.gen_range(0..proverbs.len());
        let asked_proverb = &proverbs[random_index];
        let mut options = vec![cleanup(&asked_proverb.meaning)];

        while options.len() < 4 {
            let random_index = rng.gen_range(0..proverbs.len());
            let random_proverb = &proverbs[random_index];
            if !options.contains(&random_proverb.meaning) {
                options.push(cleanup(&random_proverb.meaning));
            }
        }

        options.shuffle(&mut rng);
        quiz.add_question(Question::new(
            asked_proverb.proverb.clone(),
            options,
            cleanup(&asked_proverb.meaning),
        ));
    }

    quiz
}

fn cleanup(proverb_meaning: &str) -> String {
    remove_after_char(&remove_numbered_patterns(proverb_meaning).trim_start(), ':')
}

fn remove_numbered_patterns(input: &str) -> String {
    let re = Regex::new(r"\d+\)").unwrap(); // Matches any digit(s) followed by a closing parenthesis
    re.replace_all(input, "").to_string() // Replaces matched patterns with an empty string
}

fn remove_after_char(input: &str, delimiter: char) -> String {
    // Split at the first occurrence of `delimiter` and take the first part
    input.split(delimiter).next().unwrap_or("").to_string()
}
