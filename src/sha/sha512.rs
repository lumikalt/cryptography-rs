pub struct Sha512 {
    pub hash: [u64; 8],
    state: Vec<[u64; 16]>,
    schedule: [u64; 80],
}

impl Sha512 {
    pub fn new(msg: &[u8]) -> Sha512 {
        Sha512 {
            hash: [
                0x6A09E667F3BCC908,
                0xBB67AE8584CAA73B,
                0x3C6EF372FE94F82B,
                0xA54FF53A5F1D36F1,
                0x510E527FADE682D1,
                0x9B05688C2B3E6C1F,
                0x1F83D9ABFB41BD6B,
                0x5BE0CD19137E2179,
            ],
            state: preprocess(msg),
            schedule: [0u64; 80],
        }
    }

    pub fn result(&mut self) -> [u8; 64] {
        self.compute();

        self.hash
            .map(|x| x.to_be_bytes())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn compute(&mut self) {
        for i in 1..=self.state.len() {
            self.schedule = prepare_schedule(self.schedule, self.state[i - 1]);

            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h]: [u64; 8] = self.hash;

            for (t, k_t) in K.iter().enumerate() {
                let t1 = h
                    .wrapping_add(Σ_1(e))
                    .wrapping_add(ch(e, f, g))
                    .wrapping_add(*k_t)
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
    }
}

fn preprocess(msg: &[u8]) -> Vec<[u64; 16]> {
    let len = msg.len() + 128 - msg.len() % 128;
    let mut res = vec![0; len + 128 * (msg.len() % 128 >= 112) as usize];

    res[..msg.len()].copy_from_slice(msg);
    res[msg.len()] = 0b10000000;

    let l = res.len();
    // Please don't try to pass a message >= 2^64 bits of length
    res[l - 8..].copy_from_slice(&(msg.len() as u64 * 8).to_be_bytes()[..]);

    res.array_chunks()
        .map(|xs| u64::from_be_bytes(*xs))
        .array_chunks()
        .collect()
}

fn prepare_schedule(mut schedule: [u64; 80], block: [u64; 16]) -> [u64; 80] {
    schedule[..16].copy_from_slice(&block);

    for t in 16..80 {
        schedule[t] = σ_1(schedule[t - 2])
            .wrapping_add(schedule[t - 7])
            .wrapping_add(σ_0(schedule[t - 15]))
            .wrapping_add(schedule[t - 16])
    }

    schedule
}

// Functions:

fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}

fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[allow(non_snake_case)]
fn Σ_0(x: u64) -> u64 {
    x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
}

#[allow(non_snake_case)]
fn Σ_1(x: u64) -> u64 {
    x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
}

fn σ_0(x: u64) -> u64 {
    x.rotate_right(1) ^ x.rotate_right(8) ^ x.wrapping_shr(7)
}

fn σ_1(x: u64) -> u64 {
    x.rotate_right(19) ^ x.rotate_right(61) ^ x.wrapping_shr(6)
}

// Constants:

static K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];
