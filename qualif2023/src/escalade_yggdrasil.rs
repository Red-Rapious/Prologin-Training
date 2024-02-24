/// * `n` - le nombre de branches de l'arbre moins 1
/// * `differences` - la liste des différences en hauteur des branches consécutives
fn le_plus_grand_saut(_n: i32, differences: Vec<i32>) -> usize {
    let mut highest_branch = 0;
    let mut highest_jump = 0;

    let mut current_branch = 0;
    let mut highest_current_jump = 0;
    for diff in differences {
        current_branch += diff;
        if diff > highest_current_jump {
            highest_current_jump = diff;
        }

        if current_branch > highest_branch {
            highest_branch = current_branch;
            highest_jump = highest_current_jump;
        }
    }

    if highest_jump > 0 {
        highest_jump as usize
    } else {
        0
    }
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let differences = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `differences` parameter");

    println!("{}", le_plus_grand_saut(n, differences));
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

#[test]
pub fn test1() {
    assert_eq!(le_plus_grand_saut(4, vec![3, 2, -5, 4]), 3);
}

#[test]
pub fn test2() {
    assert_eq!(le_plus_grand_saut(7, vec![2, 9, 18, 12, 9, 19, 1]), 19);
}

#[test]
pub fn test3() {
    assert_eq!(le_plus_grand_saut(6, vec![1, 6, -7, 9, 10, -15,]), 10);
}
