use std::{collections::HashSet, fs::File};

use jfrs::reader::{type_descriptor::TypePool, JfrReader};

pub fn type_explorer(mut reader: JfrReader<File>, type_name: &str) {
    for (mut reader, chunk) in reader.chunks().flatten() {
        let tp = &chunk.metadata.type_pool;
        let event = reader
            .events(&chunk)
            .flatten()
            .filter(|e| e.class.name() == type_name)
            .next()
            .unwrap();
        let mut visited = HashSet::<i64>::new();
        print_type(tp, &mut visited, 0, event.class.class_id);
    }
}

fn print_type(tp: &TypePool, visited: &mut HashSet<i64>, indent: usize, class_id: i64) {
    if let Some(e_type) = tp.get(class_id) {
        println!("{}", e_type.name());
        for f in &e_type.fields {
            let tn = tp.get(f.class_id).map(|t| t.name());
            println!("{}- {}: {:?}", "  ".repeat(indent), f.name(), tn);
        }
        for f in &e_type.fields {
            if visited.insert(f.class_id) {
                print_type(tp, visited, indent + 1, f.class_id);
            }
        }
    }
}
