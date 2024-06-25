fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    // println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", description);

    // count the number of occurences of each vowel in sentence
    let (mut a, mut e, mut i, mut o, mut u) = (0, 0, 0, 0, 0);

    for c in sentence.chars() {
        match c {
            'a' => a += 1,
            'e' => e += 1,
            'i' => i += 1,
            'o' => o += 1,
            'u' => u += 1,
            _ => continue,
        }
    }

    println!("a: {}, e: {}, i: {}, o: {}, u: {}", a, e, i, o, u);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);
}
