fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let n: usize = s.trim().parse().unwrap();
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let x: i32 = s.trim().parse().unwrap();
        a.push(x);
    }
    shell_sort(&mut a, n);
    for v in a {
        println!("{}", v);
    }
}

fn insertion_sort(a: &mut Vec<i32>, n: usize, g: usize) -> usize {
    let mut cnt = 0;
    for i in g..n {
        let v = a[i];
        let mut j = i;
        while j >= g && a[j - g] > v {
            a[j] = a[j - g];
            j -= g;
            cnt += 1;
        }
        a[j] = v;
    }
    cnt
}

fn shell_sort(a: &mut Vec<i32>, n: usize) {
    let mut g: Vec<usize> = Vec::new();
    let mut h: usize = 1;
    while h <= n {
        g.push(h);
        h = 3 * h + 1;
    }
    g.reverse();
    let m = g.len();
    println!("{}", m);
    let s = g
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
    let mut cnt = 0;
    for i in 0..m {
        cnt += insertion_sort(a, n, g[i]);
    }
    println!("{}", cnt);
}
