extern crate tera;

use tera::{Tera, Context};

fn main() {
    let tera = Tera::new("templates/**/*").unwrap();
    let text = tera.render("index.txt", Context::new()).unwrap();
    println!("{}", &text);
}
