fn factors(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 1..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            if n / i == i {
                result.push(i);
            } else {
                result.push(i);
                result.push(n / i);
            }
        }
    }
    result
}

fn main() {
    for part1 in [true, false] {
        let mut house = 1;
        loop {
            let mut sum = 0;
            for e in factors(house) {
                if part1 || house <= e * 50 {
                    sum += e * (if part1 { 10 } else { 11 });
                }
            }
            if sum >= 29000000 {
                break;
            }
            house += 1;
        }

        println!("{}", house);
    }
}
