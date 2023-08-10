use std::collections::HashMap;

/// * `n` - le nombre de caractères gravés sur la stèle
/// * `contenu` - le texte gravé sur la stèle
/// * `occurences` - la liste contenant les nombres d'occurrences des lettres de A à Z
fn contenu_dechiffre(_n: i32, contenu: Vec<char>, occurences: Vec<i32>) -> Vec<char> {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut table_occurences: HashMap<char, i32> = HashMap::new();

    for c in contenu.iter() {
        match table_occurences.get(&c) {
            None => table_occurences.insert(*c, 1),
            Some(occ) => table_occurences.insert(*c, occ + 1)
        };
    }

    let mut translator: HashMap<char, char> = HashMap::new();
    for (lower, occ_low) in table_occurences.iter() {
        let upper = occurences.iter().position(|occ_up| *occ_up == *occ_low).unwrap();
        translator.insert(*lower, alphabet[upper]);
    }

    contenu
        .iter()
        .map(|lower| *translator.get(&lower).unwrap())
        .collect()
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let contenu = read_line(&mut buffer).chars().collect();

    let occurences = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `occurences` parameter");

    println!("{}", contenu_dechiffre(n, contenu, occurences).iter().collect::<String>());
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
    assert_eq!(contenu_dechiffre(
        28,
        "hbbbbttttoootttzzqqqqqqmmmmm".chars().collect(),
        "0 0 0 0 0 0 2 0 6 0 0 3 0 5 7 1 0 4 0 0 0 0 0 0 0 0".split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>().expect("")
    ), "PRRRROOOOLLLOOOGGIIIIIINNNNN".chars().collect::<Vec<_>>());
}

#[test]
fn test2() {
    assert_eq!(contenu_dechiffre(
        17,
        "ybkqqybkqqybbbbbb".chars().collect(),
        "4 0 0 0 0 0 0 0 8 0 2 0 0 0 0 3 0 0 0 0 0 0 0 0 0 0".split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>().expect("")
    ), "PIKAAPIKAAPIIIIII".chars().collect::<Vec<_>>());
}