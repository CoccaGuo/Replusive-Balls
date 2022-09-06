extern crate rand;

use std::{f64::consts::PI, io::Write, env};
use rand::Rng;

struct Point {
    theta: f64,
    phi: f64,
    r: f64,
}

struct XYZPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn unit(theta: f64, phi: f64) -> Point {
        Point { theta, phi, r: 1.0 }
    }

    fn to_xyz(&self) -> XYZPoint {
        let x = self.r * self.theta.sin() * self.phi.cos();
        let y = self.r * self.theta.sin() * self.phi.sin();
        let z = self.r * self.theta.cos();
        XYZPoint { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        let p1 = self.to_xyz();
        let p2 = other.to_xyz();
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
    }

    fn copy(&self) -> Point {
        Point {
            theta: self.theta,
            phi: self.phi,
            r: self.r,
        }
    }
}

fn potential(p1: &Point, p2: &Point, k: f64) -> f64 {
    let r = p1.distance(p2);
    k / r
}

// get the potential of other points (on this spec point)
fn other_all_potential(p1: &Point, this_index: usize, all: &Vec<Point>, k: f64) -> f64 {
    let mut sum = 0.0;
    for (i, p2) in all.iter().enumerate() {
        if i != this_index {
            sum += potential(p1, p2, k);
        }
    }
    sum
}

// move_index: spec the point to move; p: PID's p
fn try_step(pts: &mut Vec<Point>, move_index: usize, p: f64, k: f64) {
    let this_point = pts[move_index].copy();
    // make trys on each axis
    let dthe = 0.01 * p;
    // + +
    let energy_pp = other_all_potential(
        &Point::unit(this_point.theta + dthe, this_point.phi + dthe),
        move_index,
        pts,
        k,
    );
    let energy_pm = other_all_potential(
        &Point::unit(this_point.theta + dthe, this_point.phi - dthe),
        move_index,
        pts,
        k,
    );
    let energy_mp = other_all_potential(
        &Point::unit(this_point.theta - dthe, this_point.phi + dthe),
        move_index,
        pts,
        k,
    );
    let energy_mm = other_all_potential(
        &Point::unit(this_point.theta - dthe, this_point.phi - dthe),
        move_index,
        pts,
        k,
    );
    if energy_pp < energy_pm && energy_pp < energy_mp && energy_pp < energy_mm {
        pts[move_index] = Point::unit(this_point.theta + dthe, this_point.phi + dthe);
    } else if energy_pm < energy_mp && energy_pm < energy_mm {
        pts[move_index] = Point::unit(this_point.theta + dthe, this_point.phi - dthe);
    } else if energy_mp < energy_mm {
        pts[move_index] = Point::unit(this_point.theta - dthe, this_point.phi + dthe);
    } else {
        pts[move_index] = Point::unit(this_point.theta - dthe, this_point.phi - dthe);
    }
}

fn run(pts: &mut Vec<Point>) {
    let file = std::fs::File::create("points.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for _i in 0..150 {
        for i in 0..pts.len() {
            try_step(pts, i, 1.0, 1.0);
            write!(&mut writer, "{} {} ", pts[i].theta, pts[i].phi).unwrap();
        }
        writeln!(&mut writer, "",).unwrap();
    }
}

fn point_gen(n: usize) -> Vec<Point> {
    let mut pts = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..n {
        pts.push(Point::unit(PI*rng.gen::<f64>(), PI*2.0*rng.gen::<f64>()));
    }
    pts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let counts = args.get(1).expect("need a number as electron counts");
    let mut point_list = point_gen(counts.parse::<usize>().expect("should be a number"));
    run(&mut point_list);
}
