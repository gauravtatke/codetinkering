use std::fs::File;
use std::io::BufReader;
use std::path;
use std::collections::HashMap;
use std::collections::HashSet;

use xml::reader::EventReader;
use xml::ParserConfig;

#[derive(Debug)]
struct DataDictionary {
    fields: HashMap<u32, FieldEntry>,
}

impl DataDictionary {
    fn new() -> Self {
        Self {
            fields_name: HashMap::new(),
            fields_type: HashMap::new(),
            fields_values: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct FieldEntry {
    number: u32,
    name: String,
    type: String,
    values: HashSet<FieldValueEntry>,
}

impl FieldEntry {
    fn new(number: u32, name: String, type: String) -> Self {
        Self {
            number,
            name,
            type,
            values: HashSet::new()
        }
    }

    fn set_valid_value(&mut self, val: FieldEntry) {
        self.values.insert(val)
    }
}

#[derive(Hash, Debug)]
struct FieldValueEntry {
    value: String,
    description: String
}


fn build_dict(dict_file: &s) -> DataDictionary {
    let config = ParserConfig::new()
                    .ignore_comments(true)
                    .trim_whitespaces(true);
}