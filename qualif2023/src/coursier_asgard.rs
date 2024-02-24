use std::collections::HashMap;

/*#[derive(PartialEq)]
enum Etat {
    PasVu,
    Vu,
}

/// * `n` - le nombre de dieux
/// * `dieux` - liste des prénoms et noms des dieux séparés par un espace
/// * `m` - nombre de passations du message
/// * `passations` - liste des échanges de message entre les dieux, les noms complets des deux dieux séparés par un espace
fn chemin_valide(n: i32, dieux: Vec<String>, _m: i32, passations: Vec<String>) -> (String, Vec<String>) {
    // correspondance noms <-> indices des dieux
    let mut indices_dieux = HashMap::new();
    for (i, dieu) in dieux.iter().enumerate() {
        indices_dieux.insert(dieu, i);
    }


    // création du graphe
    let mut graphe: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
    // ajout des arêtes
    for passation in passations {
        let mots: Vec<&str> = passation.split_whitespace().take(4).collect();
        let (p1, n1, p2, n2) = (mots[0], mots[1], mots[2], mots[3]);
        if *p1 != *p2 && *n1 != *n2 {
            // l'enveloppe a été passée entre deux dieux qui n'ont rien en commmun
            return ("NON".to_string(), vec![]);
        }

        graphe[indices_dieux[&format!("{} {}", p1, n1)]]
            .push(indices_dieux[&format!("{} {}", p2, n2)]);
        graphe[indices_dieux[&format!("{} {}", p2, n2)]]
            .push(indices_dieux[&format!("{} {}", p1, n1)]);
    }

    // détection de cycle dans un graphe pour tout sommet initial
    let mut dieux_possibles = vec![];
    for dieu_initial in &dieux {
        let mut etats: Vec<Etat> = (0..n).map(|_| Etat::PasVu).collect();
        // à changer ? il faut qu'il existe au moins un chemin possible, mais il peut néanmoins exister un cycle
        if let Ok(_) = visite_dieu(&graphe, indices_dieux[&dieu_initial], &mut etats, indices_dieux[&dieu_initial]) {
            if !etats.contains(&Etat::PasVu) {
                dieux_possibles.push(dieu_initial.clone());
            }
        }
        println!();
    }

    // la fin
    if dieux_possibles.is_empty() {
        ("NON".to_string(), vec![])
    } else {
        ("OUI".to_string(), dieux_possibles)
    }
}

fn visite_dieu(graphe: &Vec<Vec<usize>>, dieu: usize, etats: &mut Vec<Etat>, predecesseur: usize) -> Result<(), ()> {
    dbg!(dieu);
    if etats[dieu] == Etat::Vu {
        Err(())
    } else {
        etats[dieu] = Etat::Vu;
        for voisin in &graphe[dieu] {
            if *voisin != predecesseur {
                visite_dieu(graphe, *voisin, etats, dieu)?;
            }
        }
        Ok(())
    }
}*/

#[derive(PartialEq, Clone)]
enum Etat {
    PasVu,
    Vu,
}

/// * `n` - le nombre de dieux
/// * `dieux` - liste des prénoms et noms des dieux séparés par un espace
/// * `m` - nombre de passations du message
/// * `passations` - liste des échanges de message entre les dieux, les noms complets des deux dieux séparés par un espace

// VERSION AVEC BACKTRACKING
fn chemin_valide(n: i32, dieux: Vec<String>, _m: i32, passations: Vec<String>) -> (String, Vec<String>) {
    // correspondance noms <-> indices des dieux
    let mut indices_dieux = HashMap::new();
    for (i, dieu) in dieux.iter().enumerate() {
        indices_dieux.insert(dieu, i);
    }

    // création du graphe
    let mut graphe: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
    // ajout des arêtes
    for passation in passations {
        let mots: Vec<&str> = passation.split_whitespace().take(4).collect();
        let (p1, n1, p2, n2) = (mots[0], mots[1], mots[2], mots[3]);
        if *p1 != *p2 && *n1 != *n2 {
            // l'enveloppe a été passée entre deux dieux qui n'ont rien en commmun
            return ("NON".to_string(), vec![]);
        }

        graphe[indices_dieux[&format!("{} {}", p1, n1)]]
            .push(indices_dieux[&format!("{} {}", p2, n2)]);
        graphe[indices_dieux[&format!("{} {}", p2, n2)]]
            .push(indices_dieux[&format!("{} {}", p1, n1)]);
    }

    let mut dieux_possibles = vec![];
    for dieu_initial in &dieux {
        let mut etats: Vec<Etat> = (0..n).map(|_| Etat::PasVu).collect();
        if visite_dieu(&graphe, indices_dieux[&dieu_initial], &mut etats) {
            if !etats.contains(&Etat::PasVu) {
                dieux_possibles.push(dieu_initial.clone());
            }
            // TODO: vérifier que toutes les transmissions ont été utilisées (toutes les arêtes)
        }
        println!();
    }

    // la fin
    if dieux_possibles.is_empty() {
        ("NON".to_string(), vec![])
    } else {
        ("OUI".to_string(), dieux_possibles)
    }
}

fn visite_dieu(graphe: &Vec<Vec<usize>>, dieu: usize, etats: &mut Vec<Etat>) -> bool {
    dbg!(dieu);
    if etats[dieu] == Etat::Vu {
        false
    } else {
        etats[dieu] = Etat::Vu;
        let voisins = &graphe[dieu];
        let mut possible = false;
        for voisin in voisins {
            if visite_dieu(graphe, *voisin, &mut etats.clone()) {
                possible = true;
                break;
            }
        }
        possible
    }
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let dieux = (0..n).map(|_| read_line(&mut buffer).to_string()).collect();

    let m = read_line(&mut buffer)
        .parse()
        .expect("invalid `M` parameter");

    let passations = (0..m).map(|_| read_line(&mut buffer).to_string()).collect();

    let (res, dieux_initiaux) = chemin_valide(n, dieux, m, passations);
    println!("{}", res);
    for dieu in dieux_initiaux {
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
        chemin_valide(
            5,
            vec!["Gudrir Bjorg".to_string(),
                "Gudrir Idunn".to_string(),
                "Gudrir Alvis".to_string(),
                "Harvardr Idunn".to_string(),
                "Harvardr Alvis".to_string()],
            4,
            vec!["Gudrir Bjorg Gudrir Idunn".to_string(),
            "Harvardr Alvis Harvardr Idunn".to_string(),
            "Gudrir Idunn Gudrir Alvis".to_string(),
            "Gudrir Idunn Harvardr Idunn".to_string()],
        ),
        ("OUI".to_string(), vec!["Gudrir Alvis".to_string()])
    );
}

#[test]
fn test2() {
    assert_eq!(
        chemin_valide(
            6,
            vec!["Dan Randolf".to_string(),
                "Idun Viggo".to_string(),
                "Dan Torleif".to_string(),
                "Ari Viggo".to_string(),
                "Vidar Viggo".to_string(),
                "Dan Viggo".to_string()],
            5,
            vec!["Dan Viggo Vidar Viggo".to_string(),
                    "Ari Viggo Dan Viggo".to_string(),
                    "Dan Torleif Dan Randolf".to_string(),
                    "Dan Viggo Idun Viggo".to_string(),
                    "Dan Randolf Dan Viggo".to_string()]
        ),
        ("NON".to_string(), vec![])
    );
}
