use std::fs;

struct Pots {
    /// A bit vector representing the pots. 1 means there is a plant in the pot,
    /// 0 means there isn't.
    state: Vec<usize>,

    /// A copy of the bit vector `state` before [Self::step] was called
    prev_state: Vec<usize>,

    /// The ID of the pot at the beginning of the bit vector `state`
    pos: i64,
}

impl Pots {
    /// Parses the initial state into a bit vector
    fn from(initial_state: &[u8]) -> Self {
        // leave four bits empty at the beginning, so extracting bits in
        // `step()` is easier
        let mut state = vec![0];
        let mut index_last = 0;
        for (i, &b) in initial_state.iter().enumerate() {
            let r = (i + 4) % 64;
            if r == 0 {
                state.push(0);
                index_last += 1;
            }
            if b == b'#' {
                state[index_last] |= 1 << r;
            }
        }

        Self {
            state,
            prev_state: Vec::new(),
            pos: -4,
        }
    }

    /// Applies the given rules to the pots and updates [Self::state]. A copy of
    /// the state before this method was called is left in [Self::prev_state].
    fn step(&mut self, rules: &[bool; 32]) {
        // prepare new state
        std::mem::swap(&mut self.state, &mut self.prev_state);
        self.state.clear();
        self.state.push(0);
        let mut index_last = 0;

        // leave four bits empty at the beginning
        let mut j = 4;

        // skip trailing zeros so the pots always start at the same bit
        // position, regardless of `self::pos`
        let mut i = self.prev_state[0].trailing_zeros() as usize - 4;
        self.pos += i as i64 - 2;

        // truncate bit vector at the last set bit
        let len = (self.prev_state.len() - 1) * 64
            + (64 - self.prev_state[self.prev_state.len() - 1].leading_zeros() as usize);

        // apply rules and built up new state
        while i < len {
            let q = i / 64;
            let r = i % 64;

            // extract up to five bits from the state at index q and position r
            let mut w = (self.prev_state[q] >> r) & 0b11111;

            // if necessary, extract remaining bits from index q + 1
            if r >= 60 && q + 1 < self.prev_state.len() {
                w |= (self.prev_state[q + 1] & ((1 << (r - 59)) - 1)) << (64 - r);
            }

            if j % 64 == 0 {
                self.state.push(0);
                index_last += 1;
                j = 0;
            }
            self.state[index_last] |= (rules[w] as usize) << j;

            j += 1;
            i += 1;
        }
    }

    /// Returns the sum of the numbers of all pots containing plants
    fn sum(&self) -> i64 {
        let mut result = 0;
        for (i, &(mut s)) in self.state.iter().enumerate() {
            while s > 0 {
                // select LSB and reset it
                let j = s.trailing_zeros() as i64;
                s &= s - 1;

                result += i as i64 * 64 + j + self.pos;
            }
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();

    // parse initial state
    let initial_state = lines.next().unwrap();
    let initial_state = &initial_state[15..];
    let mut pots = Pots::from(initial_state.as_bytes());

    // parse rules into a table with all possible 2^5 bit patterns
    lines.next();
    let mut rules = [false; 32];
    for l in lines {
        let bytes = l.as_bytes();
        let from = &bytes[0..5];
        let to = bytes[9];
        if to == b'#' {
            let mut p = 0;
            for (i, &b) in from.iter().enumerate() {
                if b == b'#' {
                    p |= 1 << i;
                }
            }
            rules[p] = true;
        }
    }

    // part 1 - simulate the first 20 steps
    let mut steps = 0_i64;
    while steps < 20 {
        pots.step(&rules);
        steps += 1;
    }
    println!("{}", pots.sum());

    // part 2 - only simulate until the pattern repeats
    let mut prev_pos = 0;
    while steps < 50_000_000_000 {
        prev_pos = pots.pos;
        pots.step(&rules);
        steps += 1;
        if pots.state == pots.prev_state {
            // pattern has repeated
            break;
        }
    }

    // extrapolate to 50 billion steps
    pots.pos += (pots.pos - prev_pos) * (50_000_000_000 - steps);
    println!("{}", pots.sum());
}
