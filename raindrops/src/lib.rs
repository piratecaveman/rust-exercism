pub fn raindrops(n: u32) -> String
{
    let mut result: String = String::new();

    if matches!(n % 3, 0) { result.push_str("Pling"); };
    if matches!(n % 5, 0) { result.push_str("Plang"); };
    if matches!(n % 7, 0) { result.push_str("Plong"); };
    if result.is_empty() { return n.to_string(); };

    return result;
}
