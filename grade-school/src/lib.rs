// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::BTreeMap;
#[allow(clippy::new_without_default)]
pub struct School (BTreeMap<u32,Vec<String>>)

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .and_modify(|students| students.insert(String::from(student)));
            .or_insert(vec![String::from(student)]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade){
            Some(students) => {
                let mut students = students.iter().map(|s| s.clone()).collect::<Vec<String>>()
                students.sort();
                students
            },
            None => vec![],
        }
    }
}
