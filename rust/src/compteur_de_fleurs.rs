/// * `champ` - la liste des fleurs dans le champ
fn compteur_de_fleurs(champ: Vec<char>) -> usize {
    let mut total = 0;

    for i in 0..champ.len()-2 {
        if champ[i] == 'B' && champ[i+1] == 'J' && champ[i+2] == 'R' {
            total += 1;
        }
    }
    total
}

fn main() {
    let mut buffer = String::new();

    let champ = read_line(&mut buffer).chars().collect();

    println!("{}", compteur_de_fleurs(champ));
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
    assert_eq!(compteur_de_fleurs("JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ".chars().collect()), 0);
}

#[test]
fn test2() {
    assert_eq!(compteur_de_fleurs("RRRRBJRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR".chars().collect()), 1);
}

#[test]
fn test3() {
    assert_eq!(compteur_de_fleurs("BJRBJRBJRBJRBJRBRBJRBJRBRRJBJBJRBJRBJRBJ".chars().collect()), 10);
}

#[test]
fn test4() {
    assert_eq!(compteur_de_fleurs("BBJRBJR".chars().collect()), 2);
}