pub fn build_proverb(list: &[&str]) -> String
{

    if matches!(list.len(), 0) { return String::new(); };

    let line = String::from("For want of a {first} the {second} was lost.");
    let end = String::from("And all for the want of a {first}.");
    let mut words = list.windows(2).peekable();

    let mut result: Vec<String> = vec![];
    while words.peek().is_some()
    {
        let current = words.next().unwrap();
        let new_line = line.replace("{first}", current[0]).replace("{second}", current[1]);
        result.push(new_line);
    }

    result.push(end.replace("{first}", list[0]));
    return result.join("\n");
}
