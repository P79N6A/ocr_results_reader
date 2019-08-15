extern crate serde_json;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::{HashMap, HashSet};

use serde_json::{Result, Value};

fn main() {
    let f = File::open("./data/tmp.txt").unwrap();
    let f = BufReader::new(f);

    let mut ret: HashMap<String, HashSet<String>> = HashMap::new();

    let mut i = 0;
    for _line in f.lines() {
        i += 1;
        let line = _line.unwrap().trim().to_string();
        if line.len() == 0 {continue}
        let mut lines = line.split_whitespace();
        let albumid = lines.next().unwrap().to_string();
        let tvid = lines.next().unwrap().to_string();
        let item = ret.entry(albumid).or_insert(HashSet::new());
        item.insert(tvid);
    }
    println!("lines: {}, len of ret: {}", i, ret["222281201"].len());
}
