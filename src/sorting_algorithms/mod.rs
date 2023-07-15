use termion::clear::UntilNewline;

use crate::array_state_event::ArrayStateEventStore;

pub enum SortingAlgorithm {
    BubbleSort,
    InsertionSort,
    SelectionSort,
    // MergeSort,
}

impl SortingAlgorithm {
    pub fn get_name(&self) -> &str {
        match self {
            SortingAlgorithm::BubbleSort => "Bubble Sort",
            SortingAlgorithm::InsertionSort => "Insertion Sort",
            SortingAlgorithm::SelectionSort => "Selection Sort",
            // SortingAlgorithm::MergeSort => "Merge Sort",
        }
    }
    pub fn sort(&self, arr: &mut ArrayStateEventStore<u64>) {
        match self {
            SortingAlgorithm::BubbleSort => {
                bubble_sort(arr);
            }
            SortingAlgorithm::InsertionSort => {
                insertion_sort(arr);
            }
            SortingAlgorithm::SelectionSort => {
                selection_sort(arr);
            } // SortingAlgorithm::MergeSort => {
              //     merge_sort(arr);
              // }
        }
    }
}

fn bubble_sort(arr: &mut ArrayStateEventStore<u64>) {
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn insertion_sort(arr: &mut ArrayStateEventStore<u64>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn selection_sort(arr: &mut ArrayStateEventStore<u64>) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j
            }
        }
        if min != i {
            arr.swap(i, min);
        }
    }
}

// fn merge(
//     left: &ArrayStateEventStore<u64>,
//     right: &ArrayStateEventStore<u64>,
// ) -> ArrayStateEventStore<u64> {
//     let mut i = 0;
//     let mut j = 0;
//     let mut merged: ArrayStateEventStore<u64> = ArrayStateEventStore::new2();
//
//     while i < left.len() && j < right.len() {
//         if left[i] < right[j] {
//             merged.push(left[i]);
//             i = i + 1;
//         } else {
//             merged.push(right[j]);
//             j = j + 1;
//         }
//     }
//
//     if i < left.len() {
//         while i < left.len() {
//             merged.push(left[i]);
//             i = i + 1;
//         }
//     }
//
//     if j < right.len() {
//         while j < right.len() {
//             merged.push(right[j]);
//             j = j + 1;
//         }
//     }
//
//     merged
// }
//
// fn merge_sort(arr: &mut ArrayStateEventStore<u64>) -> &mut ArrayStateEventStore<u64> {
//     let len = arr.len();
//     if len < 2 {
//         arr
//     } else {
//         let left = arr.split(0, len / 2);
//         let right = arr.split(len / 2 + 1, len);
//         &mut merge(&left, &right)
//     }
// }
