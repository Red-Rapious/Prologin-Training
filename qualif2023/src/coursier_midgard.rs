use std::collections::HashMap;

/// * `n` - le nombre de dieux
/// * `dieux` - liste des prénoms et noms des dieux séparés par un espace
fn afficher_chemin(n: i32, dieux: Vec<String>) -> Vec<String> {
    let mut indices_dieux = HashMap::new();
    for (i, dieu) in dieux.iter().enumerate() {
        indices_dieux.insert(dieu, i);
    }

    // création du graphe
    let mut graphe_pre: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
    let mut graphe_nom: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
    // ajout des arêtes
    for (dieu1, dieu2) in dieux.iter().zip(dieux.iter()) {
        let mots: Vec<&str> = dieu1.split_whitespace().take(2).collect();
        let (p1, n1) = (mots[0], mots[1]);

        let mots: Vec<&str> = dieu2.split_whitespace().take(2).collect();
        let (p2, n2) = (mots[0], mots[1]);

        if *p1 == *p2 {
            graphe_pre[indices_dieux[dieu1]].push(indices_dieux[dieu2]);
            graphe_pre[indices_dieux[dieu2]].push(indices_dieux[dieu1]);
        }

        if *n1 == *n2 {
            graphe_nom[indices_dieux[dieu1]].push(indices_dieux[dieu2]);
            graphe_nom[indices_dieux[dieu2]].push(indices_dieux[dieu1]);
        }
    }

    todo!()
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let dieux = (0..n)
        .map(|_| read_line(&mut buffer).to_string())
        .collect();

    for dieu in afficher_chemin(n, dieux) {
        println!("{}", dieu);
    }
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
    assert_eq!(
        afficher_chemin(3, vec![
            "Alof Amno".to_string(),
            "Baikik Brok".to_string(),
            "Alof Brok".to_string()
        ]),
        vec![
            "Alof Amno".to_string(),
            "Alof Brok".to_string(),
            "Baikik Brok".to_string()
        ]
    );
}

#[test]
fn test2() {
    assert_eq!(
        afficher_chemin(5, vec![
            "0dric Apik".to_string(),
            "0dric Brik".to_string(),
            "1dric Apik".to_string(),
            "1dric Brik".to_string(),
            "Tonb Apik".to_string()
        ]),
        vec![
            "Tonb Apik".to_string(),
            "1dric Apik".to_string(),
            "1dric Brik".to_string(),
            "0dric Brik".to_string(),
            "0dric Apik".to_string()
        ] // autres solutions possibles
    );
}

#[test]
fn test3() {
    assert_eq!(
        afficher_chemin(3, vec![
            "Seche Plusplus".to_string(),
            "Seche Seche".to_string(),
            "Seche Harpe".to_string()
        ]),
        vec!["IMPOSSIBLE".to_string()]
    );
}