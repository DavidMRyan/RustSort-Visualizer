use std::io;

fn main() {
    let mut list = [6, 3, 0, 5];
    let mut buf = String::new();
    println!("Unsorted List: {:?}", list);

    // Prompt user for input
    println!("Please Selection a Sorting Algorithm: \n\
        1.) Bubble Sort\n\
        2.) Selection Sort");
    io::stdin().read_line(&mut buf).expect("Failed to read input.");

    // Handle user selection
    let input : i32 = buf.trim().parse().unwrap();
    match input {
        1 => bubble_sort(&mut list),
        2 => selection_sort(&mut list),
        _ => println!("ERROR: Your selection was not one of the provided options!"),
    }

    println!("Sorted List: {:?}", list);
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        print!("Pass {i} -> [\t ");
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                print!("{:?} < {:?} -> Swapping!\t", arr[j + 1], arr[j]);
                arr.swap(j, j + 1);
            }
        }
        print!("]\n");
    }
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        print!("Pass {i} -> [\t ");
        let mut min_idx = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min_idx] {
                print!("NEW MIN. VAL -> {:?}[Index: {:?}]\t", arr[j], j);
                min_idx = j;
            }
        }

        print!("{:?} < {:?} -> Swapping!\t", arr[min_idx], arr[i]);
        arr.swap(min_idx, i);
        print!("]\n");
    }
}