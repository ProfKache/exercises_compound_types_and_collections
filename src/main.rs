// #![deny(clippy::all)]

mod my_types;

use my_types::LibraryItem;

fn highest_score(students: Vec<(&str, i32)>) {
    if students.is_empty() {
        println!("The list is empty");
        return;
    }
    let (mut name, mut highest) = students[0];

    for element in students.iter().skip(1) {
        if element.1 > highest {
            name = element.0;
            highest = element.1;
        }
    }

    println!("{} has the highest score: {}", name, highest);
}

fn count_items(items: Vec<LibraryItem>) {
    let mut books = 0;
    let mut magazines = 0;

    for item in &items {
        match item {
            LibraryItem::Book(_) => books += 1,
            LibraryItem::Magazine(_) => magazines += 1,
        };
    }

    println!("Books: {books}");
    println!("Magazines: {magazines}");
}

fn main() {
    println!("\nStudent Scores");

    let students = vec![("ProfKache", 50), ("Smith", 70), ("Alice", 60)];
    highest_score(students);

    let students = vec![];
    highest_score(students);

    println!("\nLibray Inventory (Books and Magazines)");

    let items = vec![
        LibraryItem::Book(String::from("Rust Programming")),
        LibraryItem::Magazine(String::from("Tech Weekly")),
        LibraryItem::Magazine(String::from("The Tech Startup")),
        LibraryItem::Book(String::from("Async Rust Programming")),
        LibraryItem::Book(String::from("Learn Rust In a Month")),
    ];

    count_items(items);
}
