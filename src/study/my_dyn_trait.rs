trait Role {
    fn hello(&mut self);
}

struct Student {
    name: String,
}

struct Teacher;

impl Role for Student {
    fn hello(&mut self) {
        println!("Student hello ......");
    }
}

impl Role for Teacher {
    fn hello(&mut self) {
        println!("Teacher hello ......");
    }
}

pub fn dyn_trait1() {
    let mut list: Vec<Box<dyn Role>> = Vec::new();
    let s = Student { name: "hello".to_string() };
    let t = Teacher;
    list.push(Box::new(s));
    list.push(Box::new(t));
    // list.push(Box::new("test"));
    for mut r in list {
        r.hello();
    }
}