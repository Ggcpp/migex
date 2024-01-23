use std::fs::File;

use serde_json::Value;

fn main() {
    let file = File::open("migaku.json").unwrap();
    let values: Value = serde_json::from_reader(file).unwrap();

    for item in values.as_array().unwrap() {
        let status = item[1].as_i64().unwrap();
        if status < 1 {
            continue;
        }

        let word = item[0].as_str().unwrap();
        let word = word.split('◴').next().unwrap();

        println!("{word}");
    }
}
