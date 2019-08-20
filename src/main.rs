extern crate serde;
extern crate serde_json;
extern crate regex;

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::{File, read_dir};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Result};
use regex::Regex;

fn parse_item(item: &Value) -> (Vec<(i64, i64, i64, i64)>, Vec<String>){
    let coords = &item[0];
    let ocrs = &item[1];
    if coords == &Value::Null {return (vec![], vec![]);}

    let mut coords_vec: Vec<(i64, i64, i64, i64)> = Vec::new();
    let mut ocrs_vec: Vec<String> = Vec::new(); 
    let mut idx = 0;
    while coords[idx] != Value::Null{
        let coord = &coords[idx];

        coords_vec.push((coord[0].as_i64().unwrap(), coord[1].as_i64().unwrap(), 
                   coord[2].as_i64().unwrap(), coord[3].as_i64().unwrap()));

        ocrs_vec.push(ocrs[idx].as_str().unwrap().to_owned());
        idx += 1;
    }

    (coords_vec, ocrs_vec)
}

fn get_json_files(dir_path: &str) -> Vec<String>{
    let paths = read_dir(dir_path).unwrap();
    let mut dir_vec: Vec<String> = Vec::new();

    for entry in paths{
        let path = entry.unwrap().path();
        let path = path.as_path();
        if path.is_dir() {continue}
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if &file_name[file_name.len()-5..] != ".text" {continue}

        dir_vec.push(file_name.to_owned());
    }
    dir_vec.sort();
    dir_vec
}

fn read_json(json_file_path: &str) -> Vec<(Vec<(i64, i64, i64, i64)>, Vec<String>)>{

    let f = File::open(json_file_path).unwrap();
    let mut f = BufReader::new(f);
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect(&format!("can not read file to string: {}", json_file_path));
    let json_ret: Value = serde_json::from_str(&contents).expect(&format!("can not parse file to json: {}", json_file_path));
    
    let mut ret:Vec<(Vec<(i64, i64, i64, i64)>, Vec<String>)> = Vec::new();
    let mut idx = 0;

    while json_ret[idx] != Value::Null{
        let item = &json_ret[idx];
        let (coord_vec, ocr_vec) = parse_item(item);
        ret.push((coord_vec, ocr_vec));
        idx += 1;
    }
    ret
}



fn main() {
    let json_file_dir = "/data/yaosikai/online_results";
    let json_files = get_json_files(json_file_dir);

    let id_in_file_name = Regex::new(r"([0-9]+)_([0-9]+)_").unwrap();

    let mut albumid = "".to_owned();
    let mut tvid = "".to_owned();
    
    let f = File::create("./online_result.txt").unwrap();
    let mut f = BufWriter::new(f);

    println!("number of json files: {}", json_files.len());
    for (cnt, file_name) in json_files[4072..].iter().enumerate(){
        println!("{}/{}", cnt, json_files.len());
        match id_in_file_name.captures(&file_name){
            Some(caps) => {
                albumid = caps[1].to_owned();
                tvid = caps[2].to_owned();
            },
            None => {continue},
        }

        let ret = read_json(&format!("{}/{}", json_file_dir, file_name));

        // for (i, item) in ret.iter().enumerate(){
        //     write!(f, "{}  {}  {:^8}    {:?}\n", albumid, tvid, i + 1, item.1);
        // }
    }
}
