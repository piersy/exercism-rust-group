pub fn reverse(input: &str) -> String {
    // print!(""");
    let mut s = String::with_capacity(input.len());

    for c in input.chars().rev() {
        s.push(c);
    }
    return s;
}
