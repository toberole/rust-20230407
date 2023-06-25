#[derive(Debug)]
enum Book {
    B1,
    B2,
}

pub fn my_enum1() {
    // let b = Book::B1;
    // println!("{:?}", b);

    enum Book {
        Papery {index: u32},
        Electronic {url: String},
    }

    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}