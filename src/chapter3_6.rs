fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse::<usize>().unwrap();
    let mut a = vec![];
    for i in 0..n {
        let mut s = String::new();
        let _tmp = s.trim().parse().ok().unwrap();
        a.push(_tmp)
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
fn print_vec(v: Vec<i32>) {
    for i in 0..v.len() {
        if i != v.len() - 1 {
            print!("{} ", v[i]);
        } else {
            println!("{}", v[i]);
        }
    }
}
