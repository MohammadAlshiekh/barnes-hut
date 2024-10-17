use crate::body::Body;
use ultraviolet::Vec2;
use std::f32::consts::PI;

pub fn black_hole_scenario(n: usize) -> Vec<Body> {
    fastrand::seed(0);
    let inner_radius = 1.0; // radius 0.62 = volume ~= 1
    let outer_radius = (n as f32).cbrt() * inner_radius * 10_000.0;
    println!("outer_radius: {} parsecs", outer_radius / 3.086e+16);

    let mut bodies: Vec<Body> = Vec::with_capacity(n);

    let black_hole_density: f32 = 4e14; // 4e14 solar masses per parsec^3

    let m = black_hole_density * inner_radius.powf(3.0) * PI * 4.0 / 3.0;   
    let center = Body::new(Vec2::zero(), Vec2::zero(), m as f32, inner_radius);
    bodies.push(center);

    while bodies.len() < n {
        let a = fastrand::f32() * std::f32::consts::TAU;
        let b = fastrand::f32() * std::f32::consts::PI;
        let (sin, cos) = a.sin_cos();
        let (sinb, _cosb) = b.sin_cos();
        let pos = Vec2::new(cos * sinb, sin * sinb) * outer_radius;
        let vel = Vec2::new(-sin, cos);
        let mass = 1.0f32;
        let radius = mass.cbrt();

        bodies.push(Body::new(pos, vel, mass, radius));
    }

    bodies.sort_by(|a, b| a.pos.mag_sq().total_cmp(&b.pos.mag_sq()));
    let mut mass = 0.0;
    for i in 0..n {
        mass += bodies[i].mass;
        if bodies[i].pos == Vec2::zero() {
            continue;
        }

        let v = (mass / bodies[i].pos.mag()).sqrt();
        bodies[i].vel *= v;
    }

    bodies
}

pub fn uniform_disc(n: usize) -> Vec<Body> {
    fastrand::seed(0);
    let inner_radius = 25.0;
    let outer_radius = (n as f32).sqrt() * 5.0;

    let mut bodies: Vec<Body> = Vec::with_capacity(n);

    let m = 1e6;
    let center = Body::new(Vec2::zero(), Vec2::zero(), m as f32, inner_radius);
    bodies.push(center);

    while bodies.len() < n {
        let a = fastrand::f32() * std::f32::consts::TAU;
        let (sin, cos) = a.sin_cos();
        let t = inner_radius / outer_radius;
        let r = fastrand::f32() * (1.0 - t * t) + t * t;
        let pos = Vec2::new(cos, sin) * outer_radius * r.sqrt();
        let vel = Vec2::new(sin, -cos);
        let mass = 1.0f32;
        let radius = mass.cbrt();

        bodies.push(Body::new(pos, vel, mass, radius));
    }

    bodies.sort_by(|a, b| a.pos.mag_sq().total_cmp(&b.pos.mag_sq()));
    let mut mass = 0.0;
    for i in 0..n {
        mass += bodies[i].mass;
        if bodies[i].pos == Vec2::zero() {
            continue;
        }

        let v = (mass / bodies[i].pos.mag()).sqrt();
        bodies[i].vel *= v;
    }

    bodies
}
