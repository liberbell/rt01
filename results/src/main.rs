use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let foo = match f {
        OK(file) => file,
        Err(error) => {
            panic!("File was not found");
        }
    }
}
