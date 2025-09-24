use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    author: String,
    article: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("How to work with json in Rust"),
        author: String::from("Nishant"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("second sentence"),
            },
            Paragraph {
                name: String::from("third sentence"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is {json}")
}
