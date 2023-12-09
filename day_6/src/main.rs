fn main() {
    let input = String::from_utf8(std::fs::read("./src/input.txt").unwrap()).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let times = parse_times(&lines);
    let distances = parse_distances(&lines);
    let mut prod = 1.0;

    for i in 0..4 {
        let quad = gen_quadratic(times[i], distances[i]);
        let normal = quad.normal_solve();
        let ceil = quad.ceiling_solve();
        if normal == ceil {
            prod *= ceil-1.0;
        } else {
            prod *= ceil;
        }
    }
    println!("{prod}");
}

fn parse_times(lines: &[&str]) -> [u32; 4] {
    let cleaned = &lines[0].split_whitespace().collect::<Vec<_>>()[1..];
    let parsed: [u32; 4] = cleaned
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    parsed
}

fn parse_distances(lines: &[&str]) -> [u32; 4] {
    let cleaned = &lines[1].split_whitespace().collect::<Vec<_>>()[1..];
    let parsed: [u32; 4] = cleaned
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    parsed
}

fn gen_quadratic(time: u32, distance: u32) -> Quadratic {
    Quadratic::new(-1.0, time as f32, -1.0 * distance as f32)
}

struct Quadratic {
    a: f32,
    b: f32,
    c: f32,
}

impl Quadratic {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }

    pub fn solve_positive(&self) -> f32 {
        let numerator = -self.b + (self.b * self.b - 4.0 * self.a * self.c).sqrt();
        let denominator = 2.0 * self.a;
        return numerator / denominator;
    }

    pub fn solve_negative(&self) -> f32 {
        let numerator = -self.b - (self.b * self.b - 4.0 * self.a * self.c).sqrt();
        let denominator = 2.0 * self.a;
        return numerator / denominator;
    }

    pub fn normal_solve(&self) -> f32 {
        (self.solve_negative() - self.solve_positive()).abs()
    }

    pub fn ceiling_solve(&self) -> f32 {
        (self.solve_negative().ceil() - self.solve_positive().ceil()).abs()
    }
}
