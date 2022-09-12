use std::io;

use rand::{thread_rng, Rng};

fn main() {
    let num = thread_rng().gen_range(1..101);
    println!("{}", num);
    let mut guess = String::new();
    loop {
        println!("start{}", guess);
        println!("{}", "请猜");
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("ddd{}", guess);

        match guess.cmp(&num) {
            std::cmp::Ordering::Less => println!("Small"),
            std::cmp::Ordering::Greater => println!("Small"),
            std::cmp::Ordering::Equal => {
                println!("yes");
                break;
            }
        }
    }
}
