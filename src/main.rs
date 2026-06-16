// #![deny(clippy::all)]

mod my_types;

use my_types::LibraryItem;
use std::collections::HashMap;

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

    let mut book_titles = vec![];
    let mut magazine_titles = vec![];

    for item in &items {
        match item {
            LibraryItem::Book(title) => {
                books += 1;
                book_titles.push(title);
            }
            LibraryItem::Magazine(title) => {
                magazines += 1;
                magazine_titles.push(title);
            }
        };
    }

    println!("\nBooks: {books}");
    for title in &book_titles {
        println!(" - {}", title);
    }

    println!("Magazines: {magazines}");
    for title in &magazine_titles {
        println!(" - {}", title);
    }
}

fn count_present_students(students: HashMap<String, bool>) {
    let mut present = 0;

    for status in students.values() {
        if *status {
            present += 1
        }
    }

    println!("Present students: {}", present);
}

fn sum_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for row in &matrix {
        for item in row {
            sum += item;
        }
    }

    sum
}

fn get_enrolled_students(subjects: &HashMap<String, Vec<String>>, subject_name: &str) {
    match subjects.get(subject_name) {
        Some(students) => {
            println!("Students in {}", subject_name);
            for student in students {
                println!("{}", student);
            }
        }
        None => println!("No {} student found", subject_name),
    }
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

    // HashMaps Exercise
    let mut student_attendance: HashMap<String, bool> = HashMap::new();
    student_attendance.insert(String::from("ProfKache"), true);
    student_attendance.insert(String::from("Alice"), false);
    student_attendance.insert(String::from("Smith"), true);

    println!("\nClassroom Attendance (HashMap)");
    count_present_students(student_attendance);

    println!("\nMatrix Sum (Nested Vector)");
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("The sum of elements is: {}", sum_matrix(matrix));

    println!("\nSchool Subjects (HashMap of Vectors)");

    let mut subjects = HashMap::new();
    subjects.insert(
        String::from("Math"),
        vec![
            String::from("ProfKache"),
            String::from("Alice"),
            String::from("Bob"),
        ],
    );
    subjects.insert(
        String::from("Science"),
        vec![String::from("ProfKache"), String::from("Bob")],
    );

    get_enrolled_students(&subjects, "Math");
    get_enrolled_students(&subjects, "Science");
}
