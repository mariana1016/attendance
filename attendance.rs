#[derive(Debug)]
struct Student {
    major: String,
}
//using function pointer for higher order
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) -> Vec<Student> {
    let majors = vec!["Computer Science", "Political Science", "Computer Engineering"];

    for (student, new_major) in collection.iter_mut().zip(majors.iter()) {
        behavior(student, new_major.to_string());
    }

    collection
}

fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn main() {
    //creating 3 students, no major assigned
    let students = vec![
        Student { major: "".into() },
        Student { major: "".into() },
        Student { major: "".into() },
    ];
//using fucntion pointer we created in the beginning
    let updated_students = update_majors(students, assign_major);
    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}'s major is: {}", i + 1, student.major);
    }
}