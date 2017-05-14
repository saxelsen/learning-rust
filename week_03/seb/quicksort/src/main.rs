use std::vec::Vec;
use std::cmp::PartialOrd;

fn quicksort_int(list: &Vec<i32>, ascending: bool) -> Vec<i32> {

    let mut new_list = list.clone();
    let mut partitions = Vec::new();
    partitions.push((0, list.len()));

    let mut partition;
    let mut partition_start;
    let mut partition_end;

    while partitions.len() > 0 {
        partition = partitions.pop();
        match partition {
            Some(x) => {
                partition_start = x.0;
                partition_end = x.1;
            },
            None => {
                panic!("Cannot pop from empty list.")
            },
        }
        let pivot_index = partition_start;
        let pivot_value = new_list[pivot_index];
        let mut store_index = pivot_index + 1;

        for i in pivot_index + 1..partition_end {
            let current_value = new_list[i];
            
            let comparison = if ascending {
                current_value < pivot_value
            }   else {
                current_value > pivot_value
            };

            if comparison {
                new_list.swap(i, store_index);
                store_index += 1;
            }
        }
        new_list.swap(pivot_index, store_index - 1);

        if partition_start != store_index - 1 {
            partitions.push((partition_start, store_index - 1));
        }

        if partition_end != store_index {
            partitions.push((store_index, partition_end))
        }

    }

    new_list
}

fn quicksort<T: PartialOrd + Clone>(list: &Vec<T>, ascending: bool) -> Vec<T> {
    let mut new_list = list.clone();

    let mut partitions = Vec::new();
    partitions.push((0, list.len()));

    let mut partition;
    let mut partition_start;
    let mut partition_end;

    while partitions.len() > 0 {
        partition = partitions.pop();
        match partition {
            Some(x) => {
                partition_start = x.0;
                partition_end = x.1;
            },
            None => {
                panic!("Cannot pop from empty list.")
            },
        }
        let pivot_index = partition_start;
        let mut store_index = pivot_index + 1;

        for i in pivot_index + 1..partition_end {

            let comparison = if ascending {
                new_list[i].lt(&new_list[pivot_index])
            }   else {
                new_list[i].gt(&new_list[pivot_index])
            };

            if comparison {
                new_list.swap(i, store_index);
                store_index += 1;
            }
        }
        new_list.swap(pivot_index, store_index - 1);

        if partition_start != store_index - 1 {
            partitions.push((partition_start, store_index - 1));
        }

        if partition_end != store_index {
            partitions.push((store_index, partition_end))
        }

    }

    new_list.to_vec()
}


fn main() {
    let list = vec![5,1,-5,2,30,10,8];
    println!("\nList:");
    for i in &list {
        println!("{}", i);
    }
    let sorted = quicksort_int(&list, true);
    println!("\nSorted list:");
    for i in &sorted {
        println!("{}", i);
    }

    let list = vec!["letter B", "letter C", "letter A"];
    println!("\nGeneric list:");
    for i in &list {
        println!("{}", i);
    }
    let sorted_generic_list = quicksort(&list, true);
    println!("\nSorted generic list:");
    for i in &sorted_generic_list {
        println!("{}", i);
    };
}
