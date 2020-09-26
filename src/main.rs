mod lorem;
use lorem::Lorem;

fn main() {
    let lorem = Lorem::new();
    lorem.get_words(23, false);
}
