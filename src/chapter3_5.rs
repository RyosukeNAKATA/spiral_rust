use std::io;

fn bubble_sort(n: usize, a: &mut Vec<(usize, &str)>, result: &Vec<&str>) {
    for i in 0..n {
        for j in ((i + 1)..n).rev() {
            if a[j].0 < a[j - 1].0 {
                let temp = a[j];
                a[j] = a[j - 1];
                a[j - 1] = temp;
            }
        }
    }
    println!("{}", a.iter().map(|i| i.1).collect::<Vec<&str>>().join(" "));
    if a.iter()
        .map(|i| i.1)
        .zip(result.iter())
        .any(|pair| pair.0 != *pair.1)
    {
        println!("{}", "Not stable");
    } else {
        println!("{}", "Stable");
    }
}

fn selection_sort(n: usize, a: &mut Vec<(usize, &str)>, result: &Vec<&str>) {
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if a[j].0 < a[minj].0 {
                minj = j;
            }
        }
        if minj != i {
            let temp = a[i];
            a[i] = a[minj];
            a[minj] = temp;
        }
    }
    println!("{}", a.iter().map(|i| i.1).collect::<Vec<&str>>().join(" "));
    if a.iter()
        .map(|i| i.1)
        .zip(result.iter())
        .any(|pair| pair.0 != *pair.1)
    {
        println!("{}", "Not stable");
    } else {
        println!("{}", "Stable");
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse::<usize>().unwrap();
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).ok();
    let a: Vec<(usize, &str)> = a
        .trim()
        .split_whitespace()
        .map(|i| {
            (
                i.chars()
                    .skip(1)
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
                i,
            )
        })
        .collect();
    let mut stable_sorted = a.clone();
    stable_sorted.sort_by_key(|pair| pair.0);
    let stable_sorted = stable_sorted
        .iter()
        .map(|pair| pair.1)
        .collect::<Vec<&str>>();
    bubble_sort(n, &mut a.clone(), &stable_sorted);
    selection_sort(n, &mut a.clone(), &stable_sorted);
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
