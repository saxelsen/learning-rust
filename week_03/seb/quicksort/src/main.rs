use std::vec::Vec;

fn quicksort_int(list: &Vec<i32>) -> Vec<i32> {

    let mut newlist = list.clone();
    newlist.swap(1,2);

    for i in &newlist {
        println!("{}", i);
    }

    newlist
}

fn main() {
    let list = vec![1,2,3,4];
    println!("New list:");
    quicksort_int(&list);
    println!("Old list:");
    for i in &list {
        println!("{}", i);
    }
}
