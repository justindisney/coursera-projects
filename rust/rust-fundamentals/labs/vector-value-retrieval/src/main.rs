
fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

// function that sums all the items in a vector
fn sum_vector(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vec {
        sum += item;
    }
    sum
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let sum = sum_vector(vec);
    println!("The sum of the vector is {}", sum);
    // get_item(3);

    // Retrieve a value at a specific index
    // let third_value = vec[2];
    // println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    // let last_value = vec.last().unwrap();
    //println!("The last value in the vector is: {}", last_value);


    // let vec = vec![];
    // Retrieve the first value using pattern matching
    // match vec.first() {
    //     Some(first_value) => println!("The first value in the vector is: {}", first_value),
    //     None => println!("The vector is empty!"),
    // }
}
