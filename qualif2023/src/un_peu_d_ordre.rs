use std::collections::HashMap;

/// * `k` - le nombre magique
/// * `n` - le nombre de personnes
/// * `tailles` - la liste des tailles de chaque personne
fn ordre(k: i32, _n: i32, tailles: Vec<i32>) -> String {
    if k == 0 {
        return "NON".to_string();
    }

    let mut sorted_tailles = tailles.clone();
    sorted_tailles.sort();

    let mut sorted_map: HashMap<(i32, i32), usize> = HashMap::new();

    for (sorted_i, taille) in sorted_tailles.iter().enumerate() {
        match sorted_map.get(&(sorted_i as i32 % k, *taille)) {
            None => sorted_map.insert((sorted_i as i32 % k, *taille), 1),
            Some(n) => sorted_map.insert((sorted_i as i32 % k, *taille), n + 1),
        };
    }

    let mut normal_map = HashMap::new();
    for (normal_i, taille) in tailles.iter().enumerate() {
        match normal_map.get(&(normal_i as i32 % k, *taille)) {
            None => normal_map.insert((normal_i as i32 % k, *taille), 1),
            Some(n) => normal_map.insert((normal_i as i32 % k, *taille), n + 1),
        };
    }

    if sorted_map == normal_map {
        "OUI".to_string()
    } else {
        "NON".to_string()
    }
}

pub fn main() {
    let mut buffer = String::new();

    let k = read_line(&mut buffer)
        .parse()
        .expect("invalid `K` parameter");

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let tailles = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `tailles` parameter");

    println!("{}", ordre(k, n, tailles));
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

#[test]
fn test1() {
    assert_eq!(ordre(1, 5, vec![5, 4, 3, 2, 1]), "OUI");
}

#[test]
fn test2() {
    assert_eq!(ordre(2, 5, vec![5, 4, 3, 2, 1]), "OUI");
}

#[test]
fn test3() {
    assert_eq!(ordre(3, 5, vec![5, 4, 3, 2, 1]), "NON");
}

#[test]
fn test4() {
    assert_eq!(ordre(3, 5, vec![5, 5, 5, 5, 5]), "OUI");
}
