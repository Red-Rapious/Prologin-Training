/// * `a` - les chiffres que les jeunes aperçoivent sur un papyrus
/// * `b` - les chiffres que les jeunes aperçoivent sur le papyrus suivant
fn papyrus_restants(a: i32, b: i32) -> u32 {
    if a == b {
        return 0;
    }
    let mut vec_a: Vec<u32> = vec![];
    let mut a = a as u32;
    while a > 0 {
        vec_a.push(a % 10);
        a = a / 10;
    }
    vec_a.reverse();

    let mut vec_b: Vec<u32> = vec![];
    let mut b = b as u32;
    while b > 0 {
        vec_b.push(b % 10);
        b = b / 10;
    }
    vec_b.reverse();
    
    let mut i = 0;
    loop {
        if i >= vec_a.len() || i >= vec_b.len() {
            break;
        }
        if vec_a[vec_a.len() - 1 - i] != vec_b[vec_b.len() - 1 - i] {
            break;
        }
        i += 1;
    }

    let actuel: u32 = vec_b
        .iter()
        .take(vec_b.len() - i)
        .rev()
        .enumerate()
        .map(|(e, c)| c * u32::pow(10, e as u32))
        .sum();

    let total: u32 = vec_b
        .iter()
        .skip(vec_b.len() - i)
        .rev()
        .enumerate()
        .map(|(z, c)| c * u32::pow(10, z as u32))
        .sum();

    total - actuel
}

pub fn main() {
    let mut buffer = String::new();

    let a = read_line(&mut buffer)
        .parse()
        .expect("invalid `A` parameter");

    let b = read_line(&mut buffer)
        .parse()
        .expect("invalid `B` parameter");

    println!("{}", papyrus_restants(a, b));
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
    assert_eq!(papyrus_restants(710, 810), 2);
}

#[test]
fn test2() {
    assert_eq!(papyrus_restants(2930, 3030), 0);
}

#[test]
fn test3() {
    assert_eq!(papyrus_restants(1111, 2111), 109);
}

#[test]
fn test4() {
    assert_eq!(papyrus_restants(2, 12), 1);
}