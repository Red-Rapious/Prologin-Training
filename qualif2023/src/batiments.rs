/// * `n` - le nombre de villes
/// * `r` - le nombre de mouvements du serpent
/// * `k` - le nombre de villes impliquées dans chaque mouvement
/// * `villes` - le nombre de bâtiments cassés dans chaque ville
fn batiments(n: i32, r: i32, k: i32, villes: Vec<i32>) -> Vec<i32> {
    let mut villes = villes;
    for i in 0..r {
        let mut max = villes[(i % n) as usize];
        for j in 1..k.min(n) {
            if villes[((i + j) % n) as usize] > max {
                max = villes[((i + j) % n) as usize];
            }
        }
        villes[(i % n) as usize] = max;
    }
    villes
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let r = read_line(&mut buffer)
        .parse()
        .expect("invalid `R` parameter");

    let k = read_line(&mut buffer)
        .parse()
        .expect("invalid `K` parameter");

    let villes = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `villes` parameter");

    let bats = batiments(n, r, k, villes);
    for (i, bat) in bats.iter().enumerate() {
        if i == bats.len() - 1 {
            print!("{}\n", bat);
        } else {
            print!("{} ", bat);
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
    assert_eq!(batiments(5, 2, 3, vec![2, 4, 3, 6, 8]), vec![4, 6, 3, 6, 8]);
}

#[test]
fn test2() {
    assert_eq!(batiments(5, 2, 4, vec![2, 4, 3, 6, 8]), vec![6, 8, 3, 6, 8]);
}

#[test]
fn test3() {
    assert_eq!(batiments(5, 6, 3, vec![5, 4, 3, 2, 1]), vec![5, 4, 3, 5, 5]);
}

#[test]
fn test4() {
    assert_eq!(
        batiments(10, 6, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        vec![3, 4, 5, 6, 7, 8, 7, 8, 9, 10]
    );
}

#[test]
fn test5() {
    assert_eq!(
        batiments(
            20,
            42,
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        ),
        vec![7, 8, 7, 8, 9, 10, 10, 10, 10, 10, 10, 9, 8, 7, 6, 5, 4, 4, 5, 6]
    );
}
