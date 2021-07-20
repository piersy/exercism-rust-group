pub fn reverse(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    for i in input.chars() {
        res.insert(0, i);
    }
    res
}
