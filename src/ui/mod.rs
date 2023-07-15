use crate::array_state_event::ArrayStateEventStore;
use crate::sorting_algorithms::SortingAlgorithm;
use crate::util::StatefulList;
pub struct App {
    pub sorting_algoritms: StatefulList<SortingAlgorithm>,
    data: ArrayStateEventStore<u64>,
    generation: usize,
    sorted: bool,
    paused: bool,
}

impl<'a> App {
    pub fn new(sorting_algoritms: Vec<SortingAlgorithm>) -> Self {
        App {
            sorting_algoritms: StatefulList::with_items(sorting_algoritms),
            data: ArrayStateEventStore::new(&mut (1..50).collect::<Vec<u64>>()),
            generation: 0,
            sorted: false,
            paused: true,
        }
    }
    pub fn get_data(&self) -> Vec<(&'a str, u64)> {
        self.data
            .get_generation(self.generation)
            .unwrap_or_default()
    }

    pub fn reset(&mut self) {
        self.data = ArrayStateEventStore::new(&mut (1..50).collect::<Vec<u64>>());
        self.generation = 0;
        self.sorted = false;
        self.paused = true;
    }

    pub fn update(&mut self) {
        if self.sorted && self.data.generation_number() > self.generation && !self.paused {
            self.generation += 1;
        }
    }

    pub fn go_back(&mut self) {
        if self.sorted && 0 < self.generation {
            self.paused = true;
            self.generation -= 1;
        }
    }

    pub fn go_forward(&mut self) {
        if self.sorted && self.data.generation_number() > self.generation {
            self.paused = true;
            self.generation += 1;
        }
    }

    pub fn run_sort(&mut self) {
        self.sorting_algoritms.items[self.sorting_algoritms.state.selected().unwrap()]
            .sort(&mut self.data);
        self.sorted = true;
        self.paused = false;
    }
}
