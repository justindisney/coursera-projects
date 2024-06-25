use::std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

// Function to calculate the average of a slice of integers
fn average(numbers: &[i32]) -> f32 {
    let sum = sum(numbers);
    let count = numbers.len();
    sum as f32 / count as f32
}

fn main() {
    // Create a vector of integers
    let mut numbers = vec![];

    loop {
        println!("Enter an integer, or 'q' to stop.");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");


        // Append number to numbers vector
        if number.trim() == "q" {
            break;
        } else {
            let number: i32 = number.trim().parse()
                .expect("Invalid number");
            numbers.push(number);
        }
    }

    // Convert vector to slice
    let numbers_slice = &numbers[..];
    let sum_of_numbers = sum(numbers_slice);
    let average_of_numbers = average(numbers_slice);

    println!("The sum of the numbers is: {}", sum_of_numbers);
    println!("The average of the numbers is: {}", average_of_numbers);
}
