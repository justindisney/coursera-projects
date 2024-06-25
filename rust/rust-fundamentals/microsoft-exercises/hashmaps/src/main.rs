use std::collections::HashMap;

fn get_review<'a>(reviews: &'a HashMap<String, String>, book: &'a str) -> String {
    let review: Option<&String> = reviews.get(book);

    match review {
        Some(review) => format!("\nReview for \'{}\': {:?}", book, reviews.get(book).unwrap()),
        None => format!("No review found for \'{}\'", book),
    }
}

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Look for a specific review
    let book: &str = "Ancient Roman History";
    let mut review: String = get_review(&reviews, book);

    println!("\nReview for \'{} \': {:?}", book, review);


    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    reviews.remove(obsolete);

    review = get_review(&reviews, obsolete);

    println!("\nReview for \'{} \': {:?}", obsolete, review);
}
