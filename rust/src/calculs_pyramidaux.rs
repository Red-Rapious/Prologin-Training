
/// * `hauteur` - la hauteur de la salle
/// * `largeur` - la largeur de la salle
/// * `longueur` - la longueur de la salle
fn taille_minimale(hauteur: i32, largeur: i32, longueur: i32) -> i32 {
    hauteur - 1 + i32::max(largeur, longueur)
}

pub fn main() {
    let mut buffer = String::new();

    let hauteur = read_line(&mut buffer)
        .parse()
        .expect("invalid `hauteur` parameter");

    let largeur = read_line(&mut buffer)
        .parse()
        .expect("invalid `largeur` parameter");

    let longueur = read_line(&mut buffer)
        .parse()
        .expect("invalid `longueur` parameter");

    println!("{}", taille_minimale(hauteur, largeur, longueur));
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
    assert_eq!(taille_minimale(2, 2, 1), 3);
}

#[test]
fn test2() {
    assert_eq!(taille_minimale(4, 2, 3), 6);
}