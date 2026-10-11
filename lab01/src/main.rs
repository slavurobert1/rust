fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}

fn exercise1() {
    for n in 0..=100 {
        if is_prime(n) {
            println!("{}", n);
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn are_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn exercise2() {
    for a in 1..=100 {
        for b in (a + 1)..=100 {
            if are_coprime(a, b) {
                println!("{} si {} sunt coprime", a, b);
            }
        }
    }
}

fn exercise3() {
    for n in (1..=99).rev() {
        if n > 2 {
            println!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
            println!(
                "Take one down and pass it around, {} bottles of beer on the wall.\n",
                n - 1
            );
        } else if n == 2 {
            println!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
            println!(
                "Take one down and pass it around, {} bottle of beer on the wall.\n",
                n - 1
            );
        } else {
            println!("1 bottle of beer on the wall, 1 bottle of beer.");
            println!("Take one down and pass it around, no more bottles of beer on the wall.\n");
        }
    }

    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}

fn main() {
    println!("=== Problema 1: Numere prime ===");
    exercise1();

    println!("=== Problema 2: Numere coprime ===");
    exercise2();

    println!("=== Problema 3: 99 Bottles of Beer ===");
    exercise3();
}
