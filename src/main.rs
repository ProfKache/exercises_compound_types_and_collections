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

fn main() {
    println!("\nStudent Scores");

    let students = vec![("ProfKache", 50), ("Smith", 70), ("Alice", 60)];
    highest_score(students);

    let students = vec![];
    highest_score(students);
}
