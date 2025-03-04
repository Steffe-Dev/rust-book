fn main() {
    let a = "awe";
    let l;
    let b: String;
    {
        b = String::from("boao");
        l = longest(a, b.as_str());
    }
    println!("{l}");
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
