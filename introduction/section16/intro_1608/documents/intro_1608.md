### [Supertraits](https://doc.rust-jp.rs/rust-by-example-ja/trait/supertraits.html#supertraits)

Rustには「継承」はありませんが、他の`trait`のスーパーセットとして trait を定義することができます。例えば以下のように

~~~rust
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

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

fn main() {}
~~~

- See also

  [スーパートレイト](https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html#%E3%82%B9%E3%83%BC%E3%83%91%E3%83%BC%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6%E5%88%A5%E3%81%AE%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E5%86%85%E3%81%A7%E3%81%82%E3%82%8B%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%AE%E6%A9%9F%E8%83%BD%E3%82%92%E5%BF%85%E8%A6%81%E3%81%A8%E3%81%99%E3%82%8B)