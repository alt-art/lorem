mod lorem;
use lorem::Lorem;

fn main() {
    let lorem = Lorem::new();
    println!("{}",lorem.get_phrase(3, 20));
}
