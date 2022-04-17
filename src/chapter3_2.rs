use std::io;

#[warn(dead_code)]
fn main() {
    let n: usize = read();
    let mut a: Vec<i32> = read_vec();

    for i in 0..n - 1 {
        let mut v = a[i];
        let mut j = i - 1;
        while j >= 0 && a[j] > v {
            a[j + 1] = a[j];
            j -= 1;
        }
        a[j + 1] = v;

        for k in 0..n {
            if k != n - 1 {
                print!("{} ", a[k]);
            } else {
                println!("{}", a[k]);
            }
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
