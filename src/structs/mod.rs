#[derive(Debug)]
struct Person {
    #[allow(unused)]
    name: String,
    language: Vec<String>,
}

impl Person {
    // !rust中这样的函数叫做关联函数
    fn new(name: String, language: Vec<String>) -> Person {
        Person { name, language }
    }

    // !rust中带有self的叫做方法
    fn master(&self) {
        let mut masterlang = String::new();
        for name in &self.language {
            masterlang.push_str(name);
        }
        println!("master language = {masterlang}");
    }
}

pub fn structs() {
    let jaylen = Person::new(
        String::from("jaylen"),
        vec![String::from("Rust"), String::from(" Typescript")],
    );
    jaylen.master();
}
