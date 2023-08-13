
/// * `n` - le nombre de clients
/// * `charges` - la liste des poids du matériel de chaque soldat, en kilogrammes
fn attente_minimale(_n: i32, mut charges: Vec<i32>) -> i32 {
    charges.sort();
    
    let mut attente = 0;
    let mut total = 0;
    for c in charges {
        total += attente;
        attente += c;
    }

    total
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let charges = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `charges` parameter");

    println!("{}", attente_minimale(n, charges));
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

#[test]
fn test_1() {
    assert_eq!(attente_minimale(4, "10 3 5 7".split_whitespace()
                            .map(str::parse)
                            .collect::<Result<_, _>>()
                            .expect("invalid `charges` parameter")), 26);
}

#[test]
fn test_2() {
    assert_eq!(attente_minimale(3, "1 3 19".split_whitespace()
                            .map(str::parse)
                            .collect::<Result<_, _>>()
                            .expect("invalid `charges` parameter")), 5);
}