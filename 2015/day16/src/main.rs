use std::fs;

const WHAT_I_REMEMBER: [i32; 10] = [
    3, // children
    7, // cats
    2, // samoyeds
    3, // pomeranians
    0, // akitas
    0, // vizslas
    5, // goldfish
    3, // trees
    2, // cars
    1, // perfumes
];

#[derive(PartialEq, Eq, Clone, Copy)]
enum Properties {
    Children = 0,
    Cats = 1,
    Samoyeds = 2,
    Pomeranians = 3,
    Akitas = 4,
    Vizslas = 5,
    Goldfish = 6,
    Trees = 7,
    Cars = 8,
    Perfumes = 9,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let sues = input.lines().map(|l| {
        let (_, attrs) = l.split_once(": ").unwrap();
        attrs.split(", ").map(|a| {
            let (k, v) = a.split_once(": ").unwrap();
            let k = match k {
                "children" => Properties::Children,
                "cats" => Properties::Cars,
                "samoyeds" => Properties::Samoyeds,
                "pomeranians" => Properties::Pomeranians,
                "akitas" => Properties::Akitas,
                "vizslas" => Properties::Vizslas,
                "goldfish" => Properties::Goldfish,
                "trees" => Properties::Trees,
                "cars" => Properties::Cars,
                "perfumes" => Properties::Perfumes,
                _ => panic!("Unknown attribute: {k}"),
            };
            (k, v.parse::<i32>().unwrap())
        })
    });

    let mut max1 = 0;
    let mut max1_sue = 0;
    let mut max2 = 0;
    let mut max2_sue = 0;

    for (i, s) in sues.enumerate() {
        let mut matches1 = 0;
        let mut matches2 = 0;
        for (p, v) in s {
            if WHAT_I_REMEMBER[p as usize] == v {
                matches1 += 1;
                matches2 += 1;
            }
            if (p == Properties::Cats || p == Properties::Trees) && v > WHAT_I_REMEMBER[p as usize]
            {
                matches2 += 1;
            }
            if (p == Properties::Pomeranians || p == Properties::Goldfish)
                && v < WHAT_I_REMEMBER[p as usize]
            {
                matches2 += 1;
            }
        }
        if matches1 > max1 {
            max1 = matches1;
            max1_sue = i;
        }
        if matches2 > max2 {
            max2 = matches2;
            max2_sue = i;
        }
    }

    println!("{}", max1_sue + 1);
    println!("{}", max2_sue + 1);
}
