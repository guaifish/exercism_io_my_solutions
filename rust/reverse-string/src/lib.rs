pub fn reverse(input: &str) -> String {
    String::from_utf16(&input.chars().rev().map(|x| x as u16).collect::<Vec<u16>>()).unwrap()
}
