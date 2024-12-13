use regex::Regex;
use std::fs;
use z3::ast::Ast;
use z3::{ast, Config, Context, SatResult, Solver};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let re_button = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
        let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let mut total = 0;
        let claws = input.split("\n\n").collect::<Vec<_>>();
        for claw in claws {
            let l = claw.lines().collect::<Vec<_>>();
            let ba = l[0];
            let bb = l[1];
            let pz = l[2];

            let mba = re_button.captures(ba).unwrap();
            let bax = mba.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let bay = mba.get(2).unwrap().as_str().parse::<u64>().unwrap();

            let mbb = re_button.captures(bb).unwrap();
            let bbx = mbb.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let bby = mbb.get(2).unwrap().as_str().parse::<u64>().unwrap();

            let mpz = re_prize.captures(pz).unwrap();
            let pzx = mpz.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let pzy = mpz.get(2).unwrap().as_str().parse::<u64>().unwrap();

            let cfg = Config::new();
            let ctx = Context::new(&cfg);
            let solver = Solver::new(&ctx);

            let presses_a = ast::Int::new_const(&ctx, "presses_a").to_real();
            let presses_b = ast::Int::new_const(&ctx, "presses_b").to_real();
            let zero = ast::Int::from_i64(&ctx, 0).to_real();

            let delta = if part1 { 0 } else { 10000000000000 };

            let bax = ast::Int::from_i64(&ctx, bax as i64).to_real();
            let bay = ast::Int::from_i64(&ctx, bay as i64).to_real();
            let bbx = ast::Int::from_i64(&ctx, bbx as i64).to_real();
            let bby = ast::Int::from_i64(&ctx, bby as i64).to_real();
            let pzx = ast::Int::from_i64(&ctx, pzx as i64 + delta).to_real();
            let pzy = ast::Int::from_i64(&ctx, pzy as i64 + delta).to_real();

            let eq1 = (&presses_a * &bax + &presses_b * &bbx - &pzx)._eq(&zero);
            let eq2 = (&presses_a * &bay + &presses_b * &bby - &pzy)._eq(&zero);

            solver.assert(&eq1);
            solver.assert(&eq2);

            if let SatResult::Sat = solver.check() {
                let model = solver.get_model().unwrap();
                let presses_a_v = model
                    .eval(&presses_a, true)
                    .unwrap()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                let presses_b_v = model
                    .eval(&presses_b, true)
                    .unwrap()
                    .to_string()
                    .parse::<f64>()
                    .unwrap();
                if !part1 || (presses_a_v <= 100.0 && presses_b_v <= 100.0) {
                    total += (presses_a_v * 3.0 + presses_b_v) as u64;
                }
            }
        }

        println!("{}", total);
    }
}
