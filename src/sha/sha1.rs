pub struct Sha1 {
    pub hash: [u32; 5],
    state: Vec<[u32; 16]>,
    schedule: [u32; 80],
}

impl Sha1 {
    pub fn new(msg: &[u8]) -> Sha1 {
        Sha1 {
            hash: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            state: preprocess(msg),
            schedule: [0; 80],
        }
    }

    pub fn result(&mut self) -> [u8; 20] {
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

            let [mut a, mut b, mut c, mut d, mut e] = self.hash;

            for t in 0..80 {
                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f(t)(b, c, d))
                    .wrapping_add(e)
                    .wrapping_add(k(t))
                    .wrapping_add(self.schedule[t]);

                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            let h = self.hash;
            self.hash = [
                h[0].wrapping_add(a),
                h[1].wrapping_add(b),
                h[2].wrapping_add(c),
                h[3].wrapping_add(d),
                h[4].wrapping_add(e),
            ]
        }
    }
}

fn preprocess(msg: &[u8]) -> Vec<[u32; 16]> {
    let len = msg.len() + 64 - msg.len() % 64;
    let mut res = vec![0; len + 64 * (msg.len() % 64 >= 56) as usize];

    res[..msg.len()].copy_from_slice(msg);
    res[msg.len()] = 0b10000000;

    let l = res.len();
    res[l - 8..].copy_from_slice(&(msg.len() as u64 * 8).to_be_bytes()[..]);

    res.array_chunks()
        .map(|xs| u32::from_be_bytes(*xs))
        .array_chunks()
        .collect()
}

fn prepare_schedule(mut schedule: [u32; 80], block: [u32; 16]) -> [u32; 80] {
    schedule[..16].copy_from_slice(&block);

    for t in 16..80 {
        schedule[t] = (schedule[t - 3] ^ schedule[t - 8] ^ schedule[t - 14] ^ schedule[t - 16])
            .rotate_left(1);
    }

    schedule
}

fn f(t: usize) -> Box<dyn Fn(u32, u32, u32) -> u32> {
    Box::new(match t {
        00..=19 => |x, y, z| (x & y) ^ (!x & z),
        20..=39 => |x, y, z| x ^ y ^ z,
        40..=59 => |x, y, z| (x & y) ^ (x & z) ^ (y & z),
        60..=79 => |x, y, z| x ^ y ^ z,
        _ => unreachable!(),
    })
}

fn k(t: usize) -> u32 {
    match t {
        0..=19 => 0x5a827999,
        20..=39 => 0x6ed9eba1,
        40..=59 => 0x8f1bbcdc,
        60..=79 => 0xca62c1d6,
        _ => unreachable!(),
    }
}
