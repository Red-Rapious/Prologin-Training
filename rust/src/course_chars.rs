
/// * `p` - le nombre de tours chronométrés
/// * `tours` - la liste des tours
fn vainqueur_qualif(_p: i32, tours: Vec<Vec<i32>>) -> i32 {
    let mut min = (0, f32::MAX);
    
    for tour in tours {
        let score = {
            if tour[2] >= 10 { continue; }
            else { tour[1] as f32 * (10.0 + (tour[2] as f32))/10.0 }
        };

        dbg!(score, min);
        if score < min.1 { min = (tour[0], score) }
    }

    return min.0
}

pub fn main() {
    let mut buffer = String::new();

    let p = read_line(&mut buffer)
        .parse()
        .expect("invalid `P` parameter");

    let tours = (0..p)
        .map(|_| {
            read_line(&mut buffer)
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .expect("invalid `tours` parameter");

    println!("{}", vainqueur_qualif(p, tours));
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
    assert_eq!(
        vainqueur_qualif(5,
vec!(vec!(44, 230, 4),
vec!(77, 220, 3),
vec!(33, 207, 5),
vec!(44, 253, 0),
vec!(77, 230, 1))),44
    );
}

#[test]
fn test_2() {
    assert_eq!(
        vainqueur_qualif(3,
            
            vec!(vec!(23, 136, 12),
            vec!(18, 133, 11),
            vec!(6, 154, 10))),
        0
    );
}

#[test]
fn test_3() {
    assert_eq!(
        vainqueur_qualif(0, vec!()),
        0
    );
}