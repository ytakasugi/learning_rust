// `Person`トレイトを定義
trait Person {
    fn name(&self) -> String;
}

// StudentはPersonのスーパートレイトです。
// Studentを実装するには、Personも実装する必要があります。
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent(computer science student)は、ProgrammerとStudentの両方のスーパイトレイトです。
// CompSciStudentを実装するには、両方のサブトレイトを実装する必要があります。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

/*
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
} 

struct User;

impl Person for User {
    fn name(&self) -> String {
        String::from("Y.takasugi")
    }
}

impl Student for User {
    fn university(&self) -> String {
        String::from("Tokyo University of Science")
    }
}

impl Programmer for User {
    fn fav_language(&self) -> String {
        String::from("Rust")
    }
}

impl CompSciStudent for User {
    fn git_username(&self) -> String {
        String::from("ytakasugi")
    }
}
*/

fn main() {

}
