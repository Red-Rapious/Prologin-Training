/// * `n` - la taille initiale de la machine
/// * `x` - la machine à compacter
fn phibonacci(_n: i32, mut x: Vec<i32>) -> Vec<i32> {
    if x.len() >= 2 {
        for i in (0..x.len()-2).rev() {
            let y = x[i+2];
            x[i+2] = 0;
            x[i+1] = x[i+1] + y % 1_000_000_007;
            x[i] = x[i] + y % 1_000_000_007;
        }
    }

    x.into_iter().filter(|e| *e != 0).map(|i| i%1_000_000_007).collect()
}

pub fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let x = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `X` parameter");

    let liste = phibonacci(n, x);
    for i in liste {
        print!("{} ", i);
    }
    print!("\n");
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
        phibonacci(3, vec![3, 1, 2]),
        vec![5, 3]
    );
}

#[test]
fn test_2() {
    assert_eq!(
        phibonacci(1, vec![42]),
        vec![42]
    );
}

#[test]
fn test_3() {
    assert_eq!(
        phibonacci(4, vec![1, 1, 2, 3]),
        vec![6, 9]
    );
}

#[test]
fn test_4() {
    assert_eq!(
        phibonacci(3, vec![1, 1000000000, 9]),
        vec![10, 2]
    );
}