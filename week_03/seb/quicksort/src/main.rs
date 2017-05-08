use std::vec::Vec;

fn quicksort_int(list: &Vec<i32>, ascending: bool) -> Vec<i32> {

    let mut newlist = list.clone();
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
        let pivot_value = newlist[pivot_index];
        let mut store_index = pivot_index + 1;

        for i in pivot_index + 1..partition_end {
            let current_value = newlist[i];
            
            let comparison = if ascending {
                current_value < pivot_value
            }   else {
                current_value > pivot_value
            };

            if comparison {
                newlist.swap(i, store_index);
                store_index += 1;
            }
        }
        newlist.swap(pivot_index, store_index - 1);

        if partition_start != store_index - 1 {
            partitions.push((partition_start, store_index - 1));
        }

        if partition_end != store_index {
            partitions.push((store_index, partition_end))
        }

    }

    newlist
}

fn main() {
    let list = vec![5,1,-5,2,30,10,8];
    println!("List:");
    for i in &list {
        println!("{}", i);
    }
    let sorted = quicksort_int(&list, true);
    println!("Sorted list:");
    for i in &sorted {
        println!("{}", i);
    }
}
