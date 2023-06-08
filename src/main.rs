use christmas_tree::start;

fn main() {
    let res = start();
    match res {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}
