use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        println!("args found");
    } else {
        println!("args not found")
    }
}
