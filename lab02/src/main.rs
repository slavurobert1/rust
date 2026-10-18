fn main() {
    println!("Exercitiul 1:");
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n1(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
    println!();

    println!("Exercitiul 2:");
    let mut s2 = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (b'z' - i as u8) as char;
        add_chars_n2(&mut s2, c, 26 - i);

        i += 1;
    }

    print!("{}", s2);
}

fn add_chars_n1(mut s: String, c: char, n: usize) -> String {
    for _ in 0..n {
        s.push(c);
    }
    s
}

fn add_chars_n2(s: &mut String, c: char, n: i32) {
    for _ in 0..n {
        s.push(c);
    }
}
