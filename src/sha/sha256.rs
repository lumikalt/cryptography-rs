#[derive(Debug)]
pub struct Sha256 {
    pub hash: [u32; 8],
    state: Vec<[u32; 16]>,
    schedule: [u32; 64],
}

impl Sha256 {
    pub fn new(msg: &[u8]) -> Sha256 {
        Sha256 {
            hash: [
                0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB,
                0x5BE0CD19,
            ],
            state: preprocess(msg),
            schedule: [0u32; 64],
        }
    }

    pub fn result(&mut self) -> [u8; 32] {
        self.compute();

        self.hash
            .map(|x| x.to_be_bytes())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn compute(&mut self) -> &mut Sha256 {
        for i in 1..=self.state.len() {
            self.schedule = prepare_schedule(self.schedule, self.state[i - 1]);

            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h]: [u32; 8] = self.hash;

            for t in 0..64 {
                let t1 = h
                    .wrapping_add(Σ_1(e))
                    .wrapping_add(ch(e, f, g))
                    .wrapping_add(K[t])
                    .wrapping_add(self.schedule[t]);

                let t2 = Σ_0(a).wrapping_add(maj(a, b, c));

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }

            let prev = self.hash;
            self.hash = [
                prev[0].wrapping_add(a),
                prev[1].wrapping_add(b),
                prev[2].wrapping_add(c),
                prev[3].wrapping_add(d),
                prev[4].wrapping_add(e),
                prev[5].wrapping_add(f),
                prev[6].wrapping_add(g),
                prev[7].wrapping_add(h),
            ];
        }

        self
    }
}

fn preprocess(msg: &[u8]) -> Vec<[u32; 16]> {
    let len = msg.len() + 64 - msg.len() % 64;
    let mut res = vec![0; len + 64 * (msg.len() % 64 >= 56) as usize];

    res[..msg.len()].copy_from_slice(&msg);
    res[msg.len()] = 0b10000000;

    let l = res.len();
    res[l - 8..].copy_from_slice(&(msg.len() as u64 * 8).to_be_bytes()[..]);

    res.array_chunks()
        .map(|xs| u32::from_be_bytes(*xs))
        .array_chunks()
        .collect()
}

fn prepare_schedule(mut schedule: [u32; 64], block: [u32; 16]) -> [u32; 64] {
    schedule[..16].copy_from_slice(&block);

    for t in 16..64 {
        schedule[t] = σ_1(schedule[t - 2])
            .wrapping_add(schedule[t - 7])
            .wrapping_add(σ_0(schedule[t - 15]))
            .wrapping_add(schedule[t - 16])
    }

    schedule
}

// Functions

fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (y & z) ^ (x & z)
}

#[allow(non_snake_case)]
fn Σ_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[allow(non_snake_case)]
fn Σ_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

fn σ_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ x.wrapping_shr(3)
}

fn σ_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ x.wrapping_shr(10)
}

// Constants
static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
