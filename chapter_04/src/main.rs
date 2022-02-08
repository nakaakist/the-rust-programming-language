fn main() {
    let my_string = String::from("hello world, this is a sample code");

    for n in 0..8 {
        let word = nth_word(&my_string, n);
        println!("{}", word);
    }
}

fn nth_word(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();

    let mut count: u32 = 0;
    let mut i_start: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == n {
                return &s[i_start..i];
            } else {
                count += 1;
                i_start = i + 1;
            }
        }
    }

    if count == n {
        return &s[i_start..];
    } else {
        return "not found";
    }
}
