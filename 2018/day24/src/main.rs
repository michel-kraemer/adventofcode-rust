use std::{cell::RefCell, cmp::Reverse, fs, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
enum Damage {
    Radiation,
    Fire,
    Slashing,
    Cold,
    Bludgeoning,
}

#[derive(Clone, PartialEq, Eq)]
struct Group {
    n_units: usize,
    hit_points: usize,
    initiative: usize,
    damage_type: Damage,
    damage: usize,
    weak_to: Vec<Damage>,
    immune_to: Vec<Damage>,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.n_units * other.damage)
            .cmp(&(self.n_units * self.damage))
            .then(other.initiative.cmp(&self.initiative))
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type GroupRef = Rc<RefCell<Group>>;

fn parse_damage(d: &str) -> Damage {
    if d.starts_with("radiation") {
        Damage::Radiation
    } else if d.starts_with("fire") {
        Damage::Fire
    } else if d.starts_with("slashing") {
        Damage::Slashing
    } else if d.starts_with("cold") {
        Damage::Cold
    } else if d.starts_with("bludgeoning") {
        Damage::Bludgeoning
    } else {
        panic!("Unknown damage type: {}", d);
    }
}

fn parse_group(g: &str) -> Option<Group> {
    let parts = g.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 3 {
        return None;
    }

    let n_units = parts[0].parse::<usize>().unwrap();
    let hit_points = parts[4].parse::<usize>().unwrap();
    let damage = parts[parts.len() - 6].parse::<usize>().unwrap();
    let damage_type = parse_damage(parts[parts.len() - 5]);
    let initiative = parts.last().unwrap().parse::<usize>().unwrap();

    let mut immune_to: Vec<Damage> = Vec::new();
    let mut weak_to: Vec<Damage> = Vec::new();
    let parens = g.split(&['(', ')']).collect::<Vec<_>>();
    if parens.len() > 1 {
        let iw = parens[1].split("; ").collect::<Vec<_>>();
        for j in iw {
            let ds = j.split_whitespace().collect::<Vec<_>>();
            if j.starts_with("immune") {
                for d in &ds[2..] {
                    immune_to.push(parse_damage(d));
                }
            } else if j.starts_with("weak") {
                for d in &ds[2..] {
                    weak_to.push(parse_damage(d));
                }
            } else {
                panic!("Invalid `immune to`/`weak to` annotation");
            }
        }
    }

    Some(Group {
        n_units,
        hit_points,
        initiative,
        damage_type,
        damage,
        weak_to,
        immune_to,
    })
}

fn calc_damage(ua: &Group, ud: &Group) -> usize {
    let damage = ua.n_units * ua.damage;
    if ud.immune_to.contains(&ua.damage_type) {
        0
    } else if ud.weak_to.contains(&ua.damage_type) {
        damage * 2
    } else {
        damage
    }
}

fn assign_groups_to_attack<'a>(
    attackers: &'a [GroupRef],
    defenders: &'a [GroupRef],
) -> Vec<(&'a GroupRef, &'a GroupRef)> {
    let mut defenders = Vec::from_iter(defenders);
    attackers
        .iter()
        .filter_map(|ua| {
            let mut max_damage = 0;
            let mut i = 0;
            for (j, ud) in defenders.iter().enumerate() {
                let d = calc_damage(&ua.borrow(), &ud.borrow());
                if d > max_damage {
                    max_damage = d;
                    i = j;
                }
            }
            if max_damage > 0 {
                Some((ua, defenders.remove(i)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn combat(
    initial_immune_system: &[Group],
    initial_infection: &[Group],
    boost: usize,
) -> (usize, usize) {
    // clone immune system groups and apply boost
    let mut immune_system = initial_immune_system
        .iter()
        .map(|u| {
            let mut nu = u.clone();
            nu.damage += boost;
            Rc::new(RefCell::new(nu))
        })
        .collect::<Vec<_>>();

    // clone infection groups
    let mut infection = initial_infection
        .iter()
        .map(|u| Rc::new(RefCell::new(u.clone())))
        .collect::<Vec<_>>();

    // Combat ...
    while !immune_system.is_empty() && !infection.is_empty() {
        // Phase 1: Target selection
        immune_system.sort();
        infection.sort();

        let mut immune_attacks = assign_groups_to_attack(&immune_system, &infection);
        let mut infection_attacks = assign_groups_to_attack(&infection, &immune_system);

        // Phase 2: Attacking
        let mut attacks = Vec::new();
        attacks.append(&mut immune_attacks);
        attacks.append(&mut infection_attacks);
        attacks.sort_by_key(|a| Reverse(a.0.borrow().initiative));

        if attacks.is_empty() {
            // mexican standoff :-) (aka tie)
            break;
        }

        for (ua, ud) in attacks.iter_mut() {
            let damage = calc_damage(&ua.borrow(), &ud.borrow());
            if damage == 0 {
                continue;
            }
            let killed_units = damage / ud.borrow().hit_points;
            let remaining_units = ud.borrow().n_units.saturating_sub(killed_units);
            ud.borrow_mut().n_units = remaining_units;
        }

        immune_system.retain(|u| u.borrow().n_units > 0);
        infection.retain(|u| u.borrow().n_units > 0);
    }

    let remaining_immune_system_units = immune_system
        .into_iter()
        .map(|u| u.borrow().n_units)
        .sum::<usize>();
    let remaining_infection_units = infection
        .into_iter()
        .map(|u| u.borrow().n_units)
        .sum::<usize>();

    (remaining_immune_system_units, remaining_infection_units)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split_once("\n\n").unwrap();

    // parse immune system groups
    let mut immune_system = Vec::new();
    for l in blocks.0.lines() {
        if let Some(g) = parse_group(l) {
            immune_system.push(g);
        }
    }

    // parse infection groups
    let mut infection = Vec::new();
    for l in blocks.1.lines() {
        if let Some(g) = parse_group(l) {
            infection.push(g);
        }
    }

    // part 1
    let (remaining_immune_system_units, remaining_infection_units) =
        combat(&immune_system, &infection, 0);
    if remaining_immune_system_units > remaining_infection_units {
        println!("{}", remaining_immune_system_units);
    } else {
        println!("{}", remaining_infection_units);
    }

    // part 2
    let mut boost = 0;
    loop {
        let (remaining_immune_system_units, remaining_infection_units) =
            combat(&immune_system, &infection, boost);

        if remaining_immune_system_units > 0 && remaining_infection_units == 0 {
            println!("{}", remaining_immune_system_units);
            break;
        }

        boost += 1;
    }
}
