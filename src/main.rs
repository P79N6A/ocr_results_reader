extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct LineLabel{
    start_line: u64,
    end_line: u64,
}

// #[derive(Serialize, Deserialize)]
// struct Result{
//     albumid: HashMap<String, HashMap<String, Line_label>>,
// }

fn main() {
    let f = File::open("./data/tmp.txt").unwrap();
    let f = BufReader::new(f);

    let mut ret: HashMap<String, HashMap<String, LineLabel>> = HashMap::new();

    let mut i:u64 = 0;
    let mut prev_tvid = "0".to_owned();
    let mut prev_albumid = "0".to_owned();

    // dumy item for simplified logic
    let mut dumy_tvid: HashMap<String, LineLabel> = HashMap::new();
    dumy_tvid.insert("0".to_owned(), LineLabel{start_line: 0, end_line: 0});
    ret.insert("0".to_owned(), dumy_tvid);

    let mut cnt = 0;
    let mut albumid = "0".to_owned();;
    let mut tvid = "0".to_owned();;
    for _line in f.lines() {
        i += 1;
        let line = _line.unwrap().trim().to_owned();
        if line.len() == 0 {continue}
        let mut lines = line.split_whitespace();

        albumid = lines.next().unwrap().to_owned();
        tvid = lines.next().unwrap().to_owned();

        match (&albumid, &tvid){
            (al_id, tv_id) if al_id != &prev_albumid => {
                ret.get_mut(&prev_albumid).unwrap().get_mut(&prev_tvid).unwrap().end_line = i - 1;
                let mut new_album_item: HashMap<String, LineLabel> = HashMap::new();
                new_album_item.insert(tvid.clone(), LineLabel{start_line: i, end_line: 0});
                ret.insert(albumid.clone(), new_album_item);
                prev_tvid = tv_id.clone();
                prev_albumid = albumid.clone();
            },
            (al_id, tv_id) if al_id == &prev_albumid && tv_id != &prev_tvid => {
                ret.get_mut(al_id).unwrap().get_mut(&prev_tvid).unwrap().end_line = i - 1;

                ret.get_mut(al_id).unwrap().insert(tv_id.clone(), LineLabel{start_line: i, end_line: 0});
        
                prev_tvid = tv_id.clone();
            },
            (_, _) => {},
        }
    }
    ret.get_mut(&albumid).unwrap().get_mut(&tvid).unwrap().end_line = i - 1;
    ret.remove("0");

    println!("{}", json!(ret));
}
