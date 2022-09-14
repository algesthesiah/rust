fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    let index = get_first_world_index(&s);
}

fn get_first_world_index(s: &String) {
    let bytes = s.as_bytes();
    for (i, &v) in bytes.iter().enumerate() {
        println!("{}{}", v, v == b'e')
    }
}
