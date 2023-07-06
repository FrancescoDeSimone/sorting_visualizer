use async_stream::stream;

use futures_core::stream::Stream;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn bubble_sort<'a>() -> impl Stream<Item = Vec<(&'a str, u64)>> {
    stream! {
        let mut arr: Vec<(&str,u64)> = (0..50).map(|x| ("",x)).collect();
        arr.shuffle(&mut thread_rng());
        loop {
            let mut swapped: bool = false;
            for i in 0..arr.len() - 1 {
            if arr[i].1 > arr[i + 1].1 {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }
            yield arr.clone();
            if !swapped {
                break;
            }
        }
    }
}

pub fn selection_sort<'a>() -> impl Stream<Item = Vec<(&'a str, u64)>> {
    stream! {
        let mut arr: Vec<(&str,u64)> = (0..50).map(|x| ("",x)).collect();
        arr.shuffle(&mut thread_rng());
        for i in 0..arr.len()-1{
            let mut min =  i;
            for j in i+1..arr.len() {
                if arr[j].1 < arr[min].1 {
                    min = j
                }
            }
            if min != i {
                arr.swap(i,min);
            }
            yield arr.clone();
        }
    }
}

pub fn insertion_sort<'a>() -> impl Stream<Item = Vec<(&'a str, u64)>> {
    stream! {
        let mut arr: Vec<(&str,u64)> = (0..50).map(|x| ("",x)).collect();
        arr.shuffle(&mut thread_rng());
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
            yield arr.clone();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use futures_util::pin_mut;
    use futures_util::stream::StreamExt;
    #[tokio::test]
    async fn bubble_test() {
        let s = bubble_sort();
        let mut last = Vec::new();
        pin_mut!(s);
        while let Some(value) = s.next().await {
            last = value;
        }
        assert_eq!(
            last.iter().map(|x| x.1).collect::<Vec<u64>>(),
            (0..50).collect::<Vec<u64>>()
        );
    }
    #[tokio::test]
    async fn insertion_test() {
        let s = insertion_sort();
        let mut last = Vec::new();
        pin_mut!(s);
        while let Some(value) = s.next().await {
            last = value;
        }

        assert_eq!(
            last.iter().map(|x| x.1).collect::<Vec<u64>>(),
            (0..50).collect::<Vec<u64>>()
        );
    }

    #[tokio::test]
    async fn selection_test() {
        let s = selection_sort();
        let mut last = Vec::new();
        pin_mut!(s);
        while let Some(value) = s.next().await {
            last = value;
        }

        assert_eq!(
            last.iter().map(|x| x.1).collect::<Vec<u64>>(),
            (0..50).collect::<Vec<u64>>()
        );
    }
}
