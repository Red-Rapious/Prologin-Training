/// * `n` - le nombre de villes autour de Midgard
/// * `m` - le nombre d'années avant le Ragnarök
/// * `villes` - le nom des villes autour de Midgard, en partant de la queue de Jörmungandr

/// * `actions` - la liste des actions prochaines de Jörmungandr
fn _old_situation_finale(n: i32, _m: i32, villes: Vec<String>, actions: Vec<char>) -> Vec<String> {
    let mut villes = villes;
    let mut jorm = n - 1;
    let mut direction = 1;
    let mut belly = vec![];

    //dbg!(&villes, &belly, jorm, n, direction);
    //println!();
    for action in actions {
        //println!("{action}");
        let n = villes.len() as i32;
        match action {
            'A' => jorm = (jorm + direction + n) % n,
            'M' => {
                if direction == 1 {
                    belly.push(villes.remove(((jorm + 1) % n) as usize));
                    if (jorm + 1) % n <= jorm {
                        jorm = (jorm - 1 + n) % n;
                    }
                } else {
                    belly.push(villes.remove(jorm as usize));
                    jorm = (jorm - 1 + n) % n;
                }
            }
            'R' => direction = -direction,
            'C' => {
                if direction == 1 {
                    villes.insert(((jorm + 1) % n) as usize, belly.pop().unwrap());
                    if jorm + 1 % n <= jorm {
                        jorm = (jorm + 1) % n;
                    }
                } else {
                    villes.insert(((jorm + 1) % n) as usize, belly.pop().unwrap());
                    jorm = (jorm + 1) % n;
                }
            }
            _ => panic!("unknown action"),
        };
        //dbg!(&villes, &belly, jorm, n, direction);
        //println!();
    }

    (0..villes.len())
        .map(|i| {
            villes[(jorm + direction * i as i32 + {
                if direction == 1 {
                    1
                } else {
                    0
                }
            }) as usize
                % villes.len()]
            .clone()
        })
        .collect()
}

fn situation_finale(n: i32, _m: i32, villes: Vec<String>, actions: Vec<char>) -> Vec<String> {
    // Initialisation
    let mut pred: Vec<usize> = (0..n).map(|i| ((i - 1 + n) % n) as usize).collect();
    let mut succ: Vec<usize> = (0..n).map(|i| ((i + 1) % n) as usize).collect();

    let mut jorm = n - 1;
    let mut direction = 1;

    let mut belly: Vec<usize> = vec![];

    for action in actions {
        match action {
            'A' => {
                jorm = if direction == 1 {
                    succ[jorm as usize]
                } else {
                    pred[jorm as usize]
                } as i32
            }
            'M' => {
                if direction == 1 {
                    let v1 = jorm as usize;
                    let v2 = succ[v1];
                    let v3 = succ[v2];

                    belly.push(v2);
                    succ[v1] = v3;
                    pred[v3] = v1;
                }
                if direction == -1 {
                    let v1 = pred[jorm as usize];
                    let v2 = jorm as usize;
                    let v3 = succ[jorm as usize];

                    belly.push(v2);
                    succ[v1] = v3;
                    pred[v3] = v1;
                    jorm = v1 as i32;
                }
            }
            'R' => direction = -direction,
            'C' => {
                let v1 = jorm as usize;
                let v2 = belly.pop().unwrap();
                let v3 = succ[jorm as usize];
                succ[v1] = v2;
                pred[v2] = v1;
                succ[v2] = v3;
                pred[v3] = v2;
                if direction == -1 {
                    jorm = v2 as i32;
                }
            }
            _ => panic!("unknown action"),
        };
    }

    // build the final list
    let mut final_villes = vec![];

    if direction == 1 {
        final_villes.push(villes[succ[jorm as usize]].clone());
        let mut city = succ[succ[jorm as usize]];
        while city != succ[jorm as usize] {
            final_villes.push(villes[city].clone());
            city = succ[city];
        }
    } else {
        final_villes.push(villes[jorm as usize].clone());
        let mut city = pred[jorm as usize];
        while city != jorm as usize {
            final_villes.push(villes[city].clone());
            city = pred[city];
        }
    }
    final_villes
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let m = read_line(&mut buffer)
        .parse()
        .expect("invalid `M` parameter");

    let villes = (0..n).map(|_| read_line(&mut buffer).to_string()).collect();

    let actions = read_line(&mut buffer).chars().collect();

    for ville in situation_finale(n, m, villes, actions) {
        println!("{}", ville);
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
        situation_finale(
            4,
            5,
            vec![
                "Rostheim".to_string(),
                "Adaheim".to_string(),
                "Pascalheim".to_string(),
                "Fortrheim".to_string()
            ],
            vec!['A', 'M', 'R', 'A', 'C']
        ),
        vec![
            "Adaheim".to_string(),
            "Fortrheim".to_string(),
            "Pascalheim".to_string(),
            "Rostheim".to_string(),
        ]
    )
}

#[test]
fn test2() {
    assert_eq!(
        situation_finale(
            6,
            6,
            vec![
                "Nixheim".to_string(),
                "Haskelheim".to_string(),
                "Cobolheim".to_string(),
                "Prologheim".to_string(),
                "Delpheim".to_string(),
                "Modulheim".to_string()
            ],
            vec!['M', 'M', 'A', 'A', 'C', 'C']
        ),
        vec![
            "Nixheim".to_string(),
            "Haskelheim".to_string(),
            "Delpheim".to_string(),
            "Modulheim".to_string(),
            "Cobolheim".to_string(),
            "Prologheim".to_string(),
        ]
    )
}

#[test]
fn test3() {
    assert_eq!(
        situation_finale(
            2,
            2,
            vec!["Lispheim".to_string(), "Erlangheim".to_string(),],
            vec!['R', 'M']
        ),
        vec!["Lispheim".to_string(),]
    )
}
