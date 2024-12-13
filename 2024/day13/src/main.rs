use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut total = 0;
        let claws = input.split("\n\n").collect::<Vec<_>>();
        for claw in claws {
            let l = claw.lines().collect::<Vec<_>>();

            let (bax, bay) = l[0][12..].split_once(", ").unwrap();
            let bax = bax.parse::<f64>().unwrap();
            let bay = bay[2..].parse::<f64>().unwrap();

            let (bbx, bby) = l[1][12..].split_once(", ").unwrap();
            let bbx = bbx.parse::<f64>().unwrap();
            let bby = bby[2..].parse::<f64>().unwrap();

            let delta = if part1 { 0.0 } else { 10000000000000.0 };

            let (pzx, pzy) = l[2][9..].split_once(", ").unwrap();
            let pzx = pzx.parse::<f64>().unwrap() + delta;
            let pzy = pzy[2..].parse::<f64>().unwrap() + delta;

            let mb = (pzx * bay - pzy * bax) / (bay * bbx - bax * bby);
            let ma = (pzy - mb * bby) / bay;

            if ma.round() * bax + mb.round() * bbx == pzx
                && ma.round() * bay + mb.round() * bby == pzy
                && (!part1 || (ma <= 100.0 && mb <= 100.0))
            {
                total += ma as i64 * 3 + mb as i64;
            }
        }

        println!("{}", total);
    }
}
