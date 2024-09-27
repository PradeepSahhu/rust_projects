fn main() {
    let mut x = 5;
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    let mut x: i32 = str.trim().parse().unwrap();

    if x > 0 {
        println!("positive");
        if x % 5 == 0 {
            println!("div by 5");
            if (x / 5) % 5 == 0 {
                println!("primus");
            } else {
                if (x / 5) < 5 {
                    println!("secundus");
                } else {
                    println!("tertius");
                }
            }
        } else {
            println!("not div by 5");
        }
    } else {
        println!("negative");
    }
}
