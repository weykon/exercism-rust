
use std::collections::BTreeMap;
#[allow(clippy::new_without_default)]
pub struct School (BTreeMap<u32,Vec<String>>);
impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }
    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .and_modify(|students| students.insert(1,String::from(student)))
            .or_insert(vec![String::from(student)]);
    }
    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade){
            Some(students) => {
                let mut students = students.iter().map(|s| s.clone()).collect::<Vec<String>>();
                students.sort();
                students
            },
            None => vec![],
        }
    }
}
