fn hashe(valeur: u64, x: &mut u64, y: &mut u64, z: &mut u64) {
    *x = (*x * valeur + *z + 37) % 1000_000_007;
    *y = (*x * 13 + 36 * *y + 257) % 1000_000_009;
    *z = (*y * valeur + 4 * valeur + *x * *x + 7) % 998_244_353;
}

/// * `v` - le nombre de villes
/// * `n` - le nombre de dieux
/// * `villes` - pour chaque dieu, une chaîne de caractère binaire. Si le dieu veut le contrôle de la ville $i$, un `1` est présent dans la chaîne de caractère à la position $i$, un `0` sinon
/// * `r` - le nombre de requêtes
/// * `requetes` - pour chaque requête, une chaîne de caractère binaire représentant un sous-ensemble de villes. Si la ville $i$ est présente dans le sous-ensemble, alors un `1` est présent dans la chaîne de caractère à la position $i$, un `0` sinon
fn somme_compatible(v: i32, n: i32, villes: Vec<Vec<char>>, r: i32, requetes: Vec<Vec<char>>) -> Vec<Vec<u64>> {
    /* TODO Initialisez `X`, `Y` et `Z` à 0. Pour chaque requête $S$ : appelez
    la fonction `Hashe` avec toutes les valeurs de $R(S, \Delta)$, pour toutes
    les valeurs de $\Delta$ entre $1$ et $N - 1$, puis affichez, sur une ligne
    et séparés par un espace, les nouvelles valeurs de `X`, `Y` et `Z`.  */
    let (mut x, mut y, mut z) = (0, 0, 0);
    
    let mut resultats = Vec::with_capacity(r as usize);
    
    for requete in requetes {
        for delta in 1..n {
            #[allow(non_snake_case)]
            let mut R = 0;
            for x in 0..n {
                let dieu1 = x as usize;
                let dieu2 = (x ^ delta) as usize;
        
                let mut nb_contestees = 0;

                // TODO: optimize with bitwise
                for ville in 0..(v as usize) {
                    if requete[ville] == '1' 
                    && villes[dieu1][ville] == '1' 
                    && villes[dieu2][ville] == '1' {
                        nb_contestees += 1;
                    }
                }

                if nb_contestees % 2 == 0 {
                    R += 1;
                }
            }
            hashe(R, &mut x, &mut y, &mut z);
        }
        resultats.push(vec![x, y, z]);
    }
    
    resultats
}

pub fn main() {
    let mut buffer = String::new();

    let v = read_line(&mut buffer)
        .parse()
        .expect("invalid `V` parameter");

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let villes = (0..n)
        .map(|_| read_line(&mut buffer).chars().collect())
        .collect();

    let r = read_line(&mut buffer)
        .parse()
        .expect("invalid `R` parameter");

    let requetes = (0..r)
        .map(|_| read_line(&mut buffer).chars().collect())
        .collect();

    for ligne in somme_compatible(v, n, villes, r, requetes) {
        for (i, entier) in ligne.iter().enumerate() {
            if i != ligne.len() - 1 {
                print!("{} ", entier);
            } else {
                print!("{}\n", entier);
            }
        }
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
        somme_compatible(
            3, 8, vec![
                "100".to_string().chars().collect(),
                "010".to_string().chars().collect(),
                "101".to_string().chars().collect(),
                "111".to_string().chars().collect(),
                "000".to_string().chars().collect(),
                "001".to_string().chars().collect(),
                "101".to_string().chars().collect(),
                "011".to_string().chars().collect(),
            ], 4, vec![
                "100".to_string().chars().collect(),
                "111".to_string().chars().collect(),
                "011".to_string().chars().collect(),
                "110".to_string().chars().collect()
            ]),
        vec![
            vec![484820768, 206380747, 402838276],
            vec![794197874, 947000107, 143480926],
            vec![548437971, 267269086, 269652611],
            vec![585655789,  82295263,  23580903]
        ]
    )
}

#[test]
fn test2() {
    assert_eq!(
        somme_compatible(
            5, 4, vec![
                "11111".to_string().chars().collect(),
                "10010".to_string().chars().collect(),
                "11100".to_string().chars().collect(),
                "01111".to_string().chars().collect()
            ], 3, vec![
                "00110".to_string().chars().collect(),
                "11001".to_string().chars().collect(),
                "11111".to_string().chars().collect()
            ]),
        vec![
            vec![2002265,   27656686,  226435344],
            vec![609856233, 789880530, 261123085],
            vec![157520669, 590314477, 408682977]
        ]
    )
}