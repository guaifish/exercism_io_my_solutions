pub fn raindrops(n: u32) -> String {
    let s: String = [
        if n % 3 == 0 { "Pling" } else { "" },
        if n % 5 == 0 { "Plang" } else { "" },
        if n % 7 == 0 { "Plong" } else { "" },
    ]
    .iter()
    .flat_map(|s| s.chars())
    .collect();
    if s == "" {
        format!("{}", n)
    } else {
        s
    }
}
