use std::fmt::Display;

use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;

lazy_static! {
    static ref INIT: [[Vec<bool>; 5]; 5] = [
        [
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
        ],
        [
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
        ],
        [
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
        ],
        [
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
        ],
        [
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
            vec![false; 64],
        ],
    ];
}

/// # Sha3-256
/// This implementation ensures a state of Vec<bool> with len 1600.
///
/// Rate: 1088
/// Capacity: 512
///
/// constants:
///
/// ```rust
/// let w = bstr.len() / 25; /*OR*/ let w = state[0][0].len();
/// let l = (w as f64).log2().round() as usize;
/// ```
#[derive(Debug)]
pub struct Sha3 {
    pub bstr: Vec<bool>,
    pub state: [[Vec<bool>; 5]; 5],
}

impl Display for Sha3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.bstr
                .iter()
                .array_chunks()
                .map(|bits: [&bool; 8]| {
                    let mut x: u8 = 0;

                    bits.iter()
                        .enumerate()
                        .filter(|(_, bit)| ***bit)
                        .for_each(|(i, _)| x += 1 << i);

                    format!("{x:02x}")
                })
                .join(" ")
        )
    }
}

impl Sha3 {
    pub fn new(bstr: Vec<bool>) -> Sha3 {
        let state = Sha3::bstr_to_state(bstr.clone());

        Sha3 { state, bstr }
    }

    fn bstr_to_state(bstr: Vec<bool>) -> [[Vec<bool>; 5]; 5] {
        let w = bstr.len() / 25;
        let mut state = INIT.clone();

        for (x, y, z) in iproduct!(0..5, 0..5, 0..w) {
            state[x][y][z] = bstr[w * (5 * y + x) + z];
        }

        state
    }

    pub fn state_to_bstr(state: [[Vec<bool>; 5]; 5]) -> Vec<bool> {
        let mut output = vec![];

        for y in 0..5 {
            output.append(&mut plane(state.clone(), y).concat())
        }

        output
    }

    pub fn keccak_p(bstr: Vec<bool>, n_r: isize) -> Vec<bool> {
        let w = bstr.len() / 25;
        let l = (w as f32).log2().round() as isize;
        let mut a = Sha3::bstr_to_state(bstr);

        for i_r in (12 + 2 * l - n_r)..(12 + 2 * l - 1) {
            println!("Round {i_r}");
            a = rnd(a, i_r);
            println!("\n");
        }

        Sha3::state_to_bstr(a)
    }

    pub fn keccak_f(bstr: Vec<bool>) -> Vec<bool> {
        let w = bstr.len() / 25;
        let l = (w as f64).log2().round() as isize;

        Sha3::keccak_p(bstr, 12 + 2 * l)
    }

    pub fn sponge<F, Pad>(f: F, pad: Pad, rate: usize, bstr: Vec<bool>, d: usize) -> Vec<bool>
    where
        F: Fn(Vec<bool>) -> Vec<bool>,
        Pad: Fn(isize, isize) -> Vec<bool>,
    {
        let mut p = bstr.clone();
        p.append(&mut pad(rate as isize, bstr.len() as isize));

        let n = p.len() / rate;
        let b = 1600;
        let capacity = b - rate;

        let p = p
            .chunks_exact(rate)
            .map(|xs| xs.to_vec())
            .collect::<Vec<Vec<bool>>>();
        let mut s = vec![false; b];

        for i in 0..n {
            let mut p = p[i].clone();
            p.append(&mut vec![false; capacity]);
            s = f(s.into_iter().zip(p).map(|(a, b)| a ^ b).collect())
        }

        let mut z: Vec<bool> = vec![];

        loop {
            s.truncate(rate);
            z.append(&mut s);

            if d <= z.len() {
                z.truncate(d);
                return z;
            }

            s = f(s);
        }
    }

    pub fn keccak(&mut self, capacity: usize) -> &mut Sha3 {
        self.bstr.append(&mut vec![false, true]);

        self.bstr = Sha3::sponge(
            Sha3::keccak_f,
            pad,
            1600 - capacity,
            self.bstr.clone(),
            capacity / 2,
        );

        self
    }
}

fn rnd(state: [[Vec<bool>; 5]; 5], _i_r: isize) -> [[Vec<bool>; 5]; 5] {
    let a = theta(state);
    println!("Theta:\n{}", Sha3::new(Sha3::state_to_bstr(a.clone())));
    // let a = rho(a);
    // println!("Rho:\n{}", Sha3::new(Sha3::state_to_bstr(a.clone())));
    // let a = pi(state);
    // println!("Pi:\n{}", Sha3::new(Sha3::state_to_bstr(a.clone())));
    // let a = chi(a);
    // println!("Chi:\n{}", Sha3::new(Sha3::state_to_bstr(a.clone())));
    // let a = iota(a, i_r);
    // println!("Iota:\n{}", Sha3::new(Sha3::state_to_bstr(a.clone())));

    a
}

/// x > 0, m >= 0
fn pad(x: isize, m: isize) -> Vec<bool> {
    let j = (-m - 2).rem_euclid(x);

    let mut p = vec![true];
    p.append(&mut vec![false; j as usize]);
    p.push(true);

    p
}

// Helpers

pub fn row(state: [[Vec<bool>; 5]; 5], y: usize, z: usize) -> [bool; 5] {
    state.map(|x| x[y][z])
}

pub fn column(state: [[Vec<bool>; 5]; 5], x: usize, z: usize) -> [bool; 5] {
    state[x].clone().map(|y| y[z])
}

pub fn lane(state: [[Vec<bool>; 5]; 5], x: usize, y: usize) -> Vec<bool> {
    state[x][y].clone()
}

pub fn plane(state: [[Vec<bool>; 5]; 5], y: usize) -> [Vec<bool>; 5] {
    state.map(|x| x[y].clone())
}

pub fn slice(state: [[Vec<bool>; 5]; 5], z: usize) -> [[bool; 5]; 5] {
    state.map(|x| x.map(|y| y[z]))
}

pub fn sheet(state: [[Vec<bool>; 5]; 5], x: usize) -> [Vec<bool>; 5] {
    state[x].clone()
}

// Step Mappings
// ALL state accesses on the x and y axis are in the order:
// 3 4 0 1 2
// so, just add 2 and ensure mod 5.

pub fn theta(state: [[Vec<bool>; 5]; 5]) -> [[Vec<bool>; 5]; 5] {
    let w = state[0][0].len();
    let mut c: Vec<Vec<bool>> = vec![vec![false; w]; 5];
    let mut d: Vec<Vec<bool>> = vec![vec![false; w]; 5];
    let mut a: [[Vec<bool>; 5]; 5] = state.clone();

    for (x, z) in iproduct!(0..5, 0..w) {
        let x = (x + 2) % 5;
        c[x][z] =
            state[x][3][z] ^ state[x][4][z] ^ state[x][0][z] ^ state[x][1][z] ^ state[x][2][z];
    }

    for (x, z) in iproduct!(0..5, 0..w) {
        d[x][z] = c[(x + 5 - 1) % 5][z] ^ c[(x + 1) % 5][(z + w - 1) % w];
    }

    for (x, y, z) in iproduct!(0..5, 0..5, 0..w) {
        a[(x + 2) % 5][(y + 2) % 5][z] ^= d[x][z];
    }

    a
}

pub fn rho(state: [[Vec<bool>; 5]; 5]) -> [[Vec<bool>; 5]; 5] {
    let w = state[0][0].len();
    let mut a = INIT.clone();

    a[2][2].copy_from_slice(&state[0][0]);

    let (mut x, mut y) = (1, 0);

    for t in 0..24 {
        for z in 0..w {
            a[(x + 2) % 5][(y + 2) % 5][z] =
                state[(x + 2) % 5][(y + 2) % 5][(z + 10 * w - (t + 1) * (t + 2) / 2) % w];

            (x, y) = (y, (2 * x + 3 * y) % 5)
        }
    }

    a
}

pub fn pi(state: [[Vec<bool>; 5]; 5]) -> [[Vec<bool>; 5]; 5] {
    let w = state[0][0].len();
    let mut a = INIT.clone();

    for (x, y, z) in iproduct!(0..5, 0..5, 0..w) {
        a[(x + 2) % 5][(y + 2) % 5][z] = state[(x + 3 * y + 2) % 5][(x + 2) % 5][z];
    }

    a
}

pub fn chi(state: [[Vec<bool>; 5]; 5]) -> [[Vec<bool>; 5]; 5] {
    let w = state[0][0].len();
    let mut a = state.clone();

    for (x, y, z) in iproduct!(0..5, 0..5, 0..w) {
        a[(x + 2) % 5][(y + 2) % 5][z] ^=
            (state[(x + 3) % 5][(y + 2) % 5][z] ^ true) & state[(x + 4) % 5][(y + 2) % 5][z]
    }

    a
}

pub fn iota(state: [[Vec<bool>; 5]; 5], i_r: isize) -> [[Vec<bool>; 5]; 5] {
    let w = state[0][0].len();
    let l = (w as f64).log2().round() as isize;

    let mut a = state;

    let mut r_c = vec![false; w];

    for j in 0..l {
        r_c[2usize.pow(j as u32) - 1] = rc(j + 7 * i_r);
    }

    for (z, r_c) in r_c.iter().enumerate() {
        a[2][2][z] ^= r_c
    }

    a
}

pub fn rc(t: isize) -> bool {
    if t % 255 == 0 {
        return true;
    }

    let mut r = vec![true, false, false, false, false, false, false, false];

    for _ in 1..(t % 255) {
        r.insert(0, true);
        r[0] ^= r[8];
        r[4] ^= r[8];
        r[5] ^= r[8];
        r[6] ^= r[8];
        r.truncate(8);
    }

    r[0]
}
