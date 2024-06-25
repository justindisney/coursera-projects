// function that inserts a value at beginning a vector
fn insert_at_beginning(v: &mut Vec<i32>, val: i32) {
    v.insert(0, val);
}

// function that appends a vector to another vector
fn append_vec(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.append(v2);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    // println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    // println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    insert_at_beginning(&mut v, 29);
    // println!("{:?}", v); // Output: [29, 0, 1, 2, 3, 4, 5, 6, 7, 8]

    let mut v1 = vec![100, 200, 300];
    append_vec(&mut v, &mut v1);
    println!("{:?}", v); // Output: [29, 0, 1, 2, 3, 4, 5, 6, 7, 8, 100, 200, 300]
}
