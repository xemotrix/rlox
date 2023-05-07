use crate::value::Value;

use anyhow::Result;

const MAX_LOADF: f64 = 0.75;


use Entry::*;
#[derive(Clone)]
enum Entry {
    Empty,
    Tomb,
    Full(String, Value),
}

struct SymTable {
    table: Vec<Entry>,
    size: usize,
}

#[allow(dead_code)]
impl SymTable {
    fn new() -> Self {
        SymTable {
            table: vec![Empty; 8],
            size: 0,
        }
    }

    fn set(&mut self, key: String, value: Value) {
        let (index, has_item) = Self::find_entry(&self.table, &key);
        if !has_item {
            self.size += 1;
        }
        self.table[index] = Full(key, value);
    }

    fn get(&mut self, key: String) -> Option<Value> {
        if self.size == 0 {
            return None;
        }

        let (index, has_item) = Self::find_entry(&self.table, &key);
        if !has_item {
            return None;
        }

        if let Full(_, v) = &self.table[index] {
            return Some(v.clone());
        }
        unreachable!()
    }

    fn delete(&mut self, key: String) -> Result<()> {
        if self.size == 0 {
            anyhow::bail!("table is empty");
        }
        let (index, has_item) = Self::find_entry(&self.table, &key);
        if !has_item {
            anyhow::bail!("key not found");
        }
        self.table[index] = Tomb;
        Ok(())
    }


    fn check_resize(&mut self) {
        if (self.size as f64) < (self.table.len() as f64 * MAX_LOADF) {
            return
        }

        let new_capacity = self.table.len() * 2;
        let mut new_table: Vec<Entry> = vec![Empty; new_capacity];

        let old_elems = std::mem::take(&mut self.table)
            .into_iter()
            .filter_map(|e| {
                if let Full(k, v) = e {
                    Some((k, v))
                } else {
                    None
                }
            });

        self.size = 0;
        for (k, v) in old_elems {
            let (index, _) = Self::find_entry(&new_table, &k);
            new_table[index] = Full(k, v);
            self.size += 1;
        }
        self.table = new_table;
    }

    fn find_entry(v: &Vec<Entry>, key: &str) -> (usize, bool) {
        let mut index = Self::hash_key(key) as usize % v.len();
        loop {
            match &v[index] {
                Full(k, _) if k == key => {
                    return (index, true);
                }
                Empty => {
                    return (index, false);
                }
                Tomb | Full(..) => {
                    index = (index + 1) % v.len();
                }
            }
        }
    }

    fn hash_key(token: &str) -> u32 {
        let mut hash = 2166136261u32;
        for c in token.bytes() {
            hash ^= c as u32;
            hash *= 16777619;
        }
        hash
    }
}

#[test]
fn test_size() {
    println!("size of symtable: {}", std::mem::size_of::<SymTable>());
    println!("size of <Option<(String, Value)>>: {}", std::mem::size_of::<Option<(String, Value)>>());
}

