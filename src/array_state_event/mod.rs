use std::io::Split;
use std::ops::Index;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct ArrayStateEventStore<T: PartialOrd + Clone> {
    items: Vec<Vec<T>>,
    metadata: Vec<(usize, usize)>,
}

impl<'a, T: PartialOrd + Clone> ArrayStateEventStore<T> {
    pub fn new(arr: &mut Vec<T>) -> Self {
        arr.shuffle(&mut thread_rng());
        ArrayStateEventStore {
            items: vec![arr.clone()],
            metadata: vec![],
        }
    }

    pub fn new2() -> Self {
        ArrayStateEventStore {
            items: vec![],
            metadata: vec![],
        }
    }
    pub fn get_generation(&self, generation: usize) -> Option<Vec<(&'a str, T)>> {
        self.items
            .get(generation)
            .map(|items| items.iter().map(|item| ("", item.clone())).collect())
    }
    pub fn get_current(&self) -> Vec<T> {
        self.items.last().unwrap().clone()
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let mut c = self.get_current();
        c.swap(i, j);
        self.metadata.push((i, j));
        self.items.push(c);
    }

    pub fn len(&self) -> usize {
        self.get_current().len()
    }
    pub fn generation_number(&self) -> usize {
        self.items.len() - 1
    }
    pub fn split(&mut self, start: usize, end: usize) {
        assert!(end < self.items.last().unwrap().len());
        self.items
            .push(self.items.last().unwrap()[start..end].to_vec());
    }
}

impl<T: PartialOrd + Clone> Index<usize> for ArrayStateEventStore<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.items.last().unwrap()[i]
    }
}
