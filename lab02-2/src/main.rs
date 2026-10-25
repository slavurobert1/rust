// ---------- Part 1: borrow to read (&str) ----------

// How many times does `c` appear in `s`?
fn count_char(s: &str, c: char) -> usize {
    let mut count = 0;
    for l in s.chars() {
        if l == c {
            count += 1;
        }
    }
    count
}

// The first word of `s` (up to the first space, or all of `s`), as a slice of `s`.
// No copying: return a &str that points into `s`. Use as_bytes and a while loop.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if bytes[i] == b' ' {
            return &s[..i];
        }
        i += 1;
    }
    s
}

// A new String with the characters of `s` in reverse order.
fn reverse(s: &str) -> String {
    let mut rev = String::new();
    for c in s.chars() {
        rev.insert(0, c);
    }
    rev
}

// True if `s` reads the same backwards. Reuse reverse().
fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

// A new String made of `s` repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut repeated = String::new();
    let mut i = 0;
    while i < n {
        repeated.push_str(s);
        i += 1;
    }
    repeated
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));
}
