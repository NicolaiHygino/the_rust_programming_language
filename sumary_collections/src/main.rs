use std::collections::HashMap;
use std::fs;

fn calculate_median(numbers: &[i32]) {
    let mut number_list = Vec::from(numbers);
    number_list.sort();

    if number_list.len() % 2 == 1 {
        let odd_middle_position = (number_list.len() as f32 / 2.0 + 0.5) - 1.0;
        println!(
            "The median is: {}",
            number_list[odd_middle_position as usize]
        );
    } else {
        let middle_index = number_list.len() / 2;
        let even_middle_position = &number_list[middle_index - 1..=middle_index];

        println!(
            "The median is: {}",
            (even_middle_position[0] + even_middle_position[1]) / 2
        );
    }
}

fn pig_latin_parser(text: &str) {
    let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let word_list = text.split_whitespace();

    for word in word_list {
        if word.len() <= 1 {
            print!("{word} ");
        } else {
            let first_char = word.chars().next();
            if let Some(c) = first_char {
                if vowels.contains(&c) {
                    print!("{word}-hay ");
                } else {
                    print!("{} ", format!("{}-{}ay", &word[1..], c));
                }
            }
        }
    }
}

fn most_used_words(text: &String) -> HashMap<String, i32> {
    let mut words_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = words_map.entry(word.to_string()).or_insert(0 as i32);
        *count += 1;
    }
    words_map
}

fn main() {
    let number_list = [1, 2, 50, 12, 934, 58, 52, 92, 55, 12, 01];
    calculate_median(&number_list);

    let test = "Nicolai lindão demais e muito bonito, cara como eu gosto dele";
    pig_latin_parser(&test);

    let content = fs::read_to_string("poem.txt").unwrap();
    println!("{:#?}", most_used_words(&content));
}
