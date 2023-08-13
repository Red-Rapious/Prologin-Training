use std::collections::HashMap;

// échanger objets que le groupe n'a pas contre objets que le marchand n'a pas
// compter le nombre d'objets que le groupe a au moins en fois deux et que le marchand n'a pas
// idem réciproquement
// prendre le plus grand

/// * `n` - le nombre d'objets dont le groupe et le marchand disposent
/// * `objets_groupe` - la liste des possessions de nos aventuriers
/// * `objets_marchand` - la liste des possessions du marchand
fn nombre_echanges(_n: i32, objets_groupe: Vec<String>, objets_marchand: Vec<String>) -> i32 {
    let mut table_groupe: HashMap<String, usize> = HashMap::new();

    for objet in objets_groupe {
        match table_groupe.get(&objet) {
            None => table_groupe.insert(objet, 1),
            Some(i) => table_groupe.insert(objet, i+1)
        };
    }

    let mut table_marchand: HashMap<String, usize> = HashMap::new();
    for objet in objets_marchand {
        match table_marchand.get(&objet) {
            None => table_marchand.insert(objet, 1),
            Some(i) => table_marchand.insert(objet, i+1)
        };
    }

    let mut surplus_groupe = 0;
    for (objet, quant) in table_groupe.iter() {
        if *quant >= 2 && table_marchand.get(objet) == None {
            surplus_groupe += 1;
        }
    }

    let mut surplus_marchand = 0;
    for (objet, quant) in table_marchand.iter() {
        if *quant >= 2 && table_groupe.get(objet) == None {
            surplus_marchand += 1;
        }
    }

    i32::min(surplus_groupe, surplus_marchand)
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let objets_groupe = (0..n)
        .map(|_| read_line(&mut buffer).to_string())
        .collect();

    let objets_marchand = (0..n)
        .map(|_| read_line(&mut buffer).to_string())
        .collect();

    println!("{}", nombre_echanges(n, objets_groupe, objets_marchand));
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
    assert_eq!(nombre_echanges(
        5,
        vec!["montre".to_string(),
        "casque".to_string(),
        "montre".to_string(),
        "carte".to_string(),
        "casque".to_string()],
        vec!["poterie".to_string(),
        "bijou".to_string(),
        "bijou".to_string(),
        "poterie".to_string(),
        "bijou".to_string()]
    ), 2);
}

#[test]
fn test_2() {
    assert_eq!(nombre_echanges(
        4,
        vec!["manuscrit".to_string(),
        "manuscrit".to_string(),
        "telephone".to_string(),
        "ordinateur".to_string()],
        vec!["manuscrit".to_string(),
        "tapisserie".to_string(),
        "manuscrit".to_string(),
        "statue".to_string()]
    ), 0);
}