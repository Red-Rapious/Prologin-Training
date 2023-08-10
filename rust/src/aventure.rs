/// * `n` - le nombre de personnes dans le vaisseau
fn aventure(n: i32) -> usize {
    42
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    println!("{}", aventure(n));
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

#[test]
fn test() {
    assert_eq!(aventure(2), 42);
}
