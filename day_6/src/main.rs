fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let time = parse_time(&lines);
    let distance = parse_distance(&lines);
    let mut prod = 1.0;

    let quad = gen_quadratic(time, distance);
    let normal = quad.normal_solve();
    let ceil = quad.ceiling_solve();
    if normal == ceil {
        prod *= ceil - 1.0;
    } else {
        prod *= ceil;
    }
    println!("{prod}");
}

fn parse_time(lines: &[&str]) -> u64 {
    let cleaned = &lines[0].split_whitespace().collect::<Vec<_>>()[1..];
    let parsed = cleaned.concat().parse::<u64>().unwrap();
    parsed
}

fn parse_distance(lines: &[&str]) -> u64 {
    let cleaned = &lines[1].split_whitespace().collect::<Vec<_>>()[1..];
    let parsed = cleaned.concat().parse::<u64>().unwrap();
    parsed
}

fn gen_quadratic(time: u64, distance: u64) -> Quadratic {
    Quadratic::new(-1.0, time as f64, -1.0 * distance as f64)
}

struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratic {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn solve_positive(&self) -> f64 {
        let numerator = -self.b + (self.b * self.b - 4.0 * self.a * self.c).sqrt();
        let denominator = 2.0 * self.a;
        return numerator / denominator;
    }

    pub fn solve_negative(&self) -> f64 {
        let numerator = -self.b - (self.b * self.b - 4.0 * self.a * self.c).sqrt();
        let denominator = 2.0 * self.a;
        return numerator / denominator;
    }

    pub fn normal_solve(&self) -> f64 {
        (self.solve_negative() - self.solve_positive()).abs()
    }

    pub fn ceiling_solve(&self) -> f64 {
        (self.solve_negative().ceil() - self.solve_positive().ceil()).abs()
    }
}
