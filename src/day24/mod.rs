use std::fmt;

pub fn solve_a() -> String {
    let mut immune_system = vec![
        // 3609 units each with 2185 hit points (weak to cold, radiation) with an attack that does 5 slashing damage at initiative 20
        Stack::new(
            "imm1".to_string(),
            3609,
            2185,
            vec!["cold".to_string(), "radiation".to_string()],
            vec![],
            5,
            "slashing".to_string(),
            20,
        ),
        // 72 units each with 5294 hit points (weak to slashing; immune to radiation, cold) with an attack that does 639 fire damage at initiative 1
        Stack::new(
            "imm2".to_string(),
            72,
            5294,
            vec!["slashing".to_string()],
            vec!["radiation".to_string(), "cold".to_string()],
            639,
            "fire".to_string(),
            1,
        ),
        // 4713 units each with 6987 hit points (weak to radiation) with an attack that does 12 slashing damage at initiative 2
        Stack::new(
            "imm3".to_string(),
            4713,
            6987,
            vec!["radiation".to_string()],
            vec![],
            12,
            "slashing".to_string(),
            2,
        ),
        // 623 units each with 9745 hit points with an attack that does 137 cold damage at initiative 6
        Stack::new(
            "imm4".to_string(),
            623,
            9745,
            vec![],
            vec![],
            137,
            "cold".to_string(),
            6,
        ),
        // 1412 units each with 9165 hit points with an attack that does 52 bludgeoning damage at initiative 3
        Stack::new(
            "imm5".to_string(),
            1412,
            9165,
            vec![],
            vec![],
            52,
            "bludgeoning".to_string(),
            3,
        ),
        // 2042 units each with 7230 hit points (immune to cold, radiation) with an attack that does 25 bludgeoning damage at initiative 15
        Stack::new(
            "imm6".to_string(),
            2042,
            7230,
            vec![],
            vec!["cold".to_string(), "radiation".to_string()],
            25,
            "bludgeoning".to_string(),
            15,
        ),
        // 209 units each with 9954 hit points with an attack that does 384 cold damage at initiative 17
        Stack::new(
            "imm7".to_string(),
            209,
            9954,
            vec![],
            vec![],
            384,
            "cold".to_string(),
            17,
        ),
        // 33 units each with 6495 hit points (weak to fire) with an attack that does 1756 fire damage at initiative 7
        Stack::new(
            "imm8".to_string(),
            33,
            6495,
            vec!["fire".to_string()],
            vec![],
            1756,
            "fire".to_string(),
            7,
        ),
        // 242 units each with 6650 hit points (immune to radiation, fire) with an attack that does 239 bludgeoning damage at initiative 12
        Stack::new(
            "imm9".to_string(),
            242,
            6650,
            vec![],
            vec!["fire".to_string(), "radiation".to_string()],
            239,
            "bludgeoning".to_string(),
            12,
        ),
        // 4701 units each with 7384 hit points (immune to cold) with an attack that does 14 fire damage at initiative 9
        Stack::new(
            "imm10".to_string(),
            4701,
            7384,
            vec![],
            vec!["cold".to_string()],
            14,
            "fire".to_string(),
            9,
        ),
    ];

    let mut infection = vec![
        //4154 units each with 21287 hit points (immune to fire, slashing, cold, radiation) with an attack that does 9 fire damage at initiative 5
        Stack::new(
            "inf1".to_string(),
            4154,
            21287,
            vec![],
            vec![
                "fire".to_string(),
                "slashing".to_string(),
                "cold".to_string(),
                "radiation".to_string(),
            ],
            9,
            "fire".to_string(),
            5,
        ),
        // 2091 units each with 5531 hit points (immune to slashing) with an attack that does 5 fire damage at initiative 13
        Stack::new(
            "inf2".to_string(),
            2091,
            5531,
            vec![],
            vec!["slashing".to_string()],
            5,
            "fire".to_string(),
            13,
        ),
        // 2237 units each with 24000 hit points with an attack that does 20 fire damage at initiative 16
        Stack::new(
            "inf3".to_string(),
            2237,
            24000,
            vec![],
            vec![],
            20,
            "fire".to_string(),
            16,
        ),
        // 149 units each with 31282 hit points (weak to radiation, cold) with an attack that does 329 radiation damage at initiative 8
        Stack::new(
            "inf4".to_string(),
            149,
            31282,
            vec!["radiation".to_string(), "cold".to_string()],
            vec![],
            329,
            "radiation".to_string(),
            8,
        ),
        // 649 units each with 39642 hit points with an attack that does 108 cold damage at initiative 18
        Stack::new(
            "inf5".to_string(),
            649,
            39642,
            vec![],
            vec![],
            108,
            "cold".to_string(),
            18,
        ),
        // 108 units each with 35626 hit points (immune to radiation; weak to slashing) with an attack that does 519 cold damage at initiative 4
        Stack::new(
            "inf6".to_string(),
            108,
            35626,
            vec!["slashing".to_string()],
            vec!["radiation".to_string()],
            519,
            "cold".to_string(),
            4,
        ),
        // 1194 units each with 37567 hit points (weak to fire, radiation) with an attack that does 59 radiation damage at initiative 19
        Stack::new(
            "inf7".to_string(),
            1194,
            37567,
            vec!["fire".to_string(), "radiation".to_string()],
            vec![],
            59,
            "radiation".to_string(),
            19,
        ),
        // 2849 units each with 37603 hit points (immune to cold) with an attack that does 26 bludgeoning damage at initiative 10
        Stack::new(
            "inf8".to_string(),
            2849,
            37603,
            vec![],
            vec!["cold".to_string()],
            26,
            "bludgeoning".to_string(),
            10,
        ),
        // 451 units each with 35892 hit points (weak to slashing; immune to cold) with an attack that does 133 fire damage at initiative 14
        Stack::new(
            "inf9".to_string(),
            451,
            35892,
            vec!["slashing".to_string()],
            vec!["cold".to_string()],
            133,
            "fire".to_string(),
            14,
        ),
        // 3232 units each with 27332 hit points (weak to fire) with an attack that does 14 cold damage at initiative 11
        Stack::new(
            "inf".to_string(),
            3232,
            27332,
            vec!["fire".to_string()],
            vec![],
            14,
            "cold".to_string(),
            11,
        ),
    ];

    battle(&mut immune_system, &mut infection).0.to_string()
}

pub fn solve_b() -> String {
    let mut immune_system = vec![
        // 3609 units each with 2185 hit points (weak to cold, radiation) with an attack that does 5 slashing damage at initiative 20
        Stack::new(
            "imm1".to_string(),
            3609,
            2185,
            vec!["cold".to_string(), "radiation".to_string()],
            vec![],
            5,
            "slashing".to_string(),
            20,
        ),
        // 72 units each with 5294 hit points (weak to slashing; immune to radiation, cold) with an attack that does 639 fire damage at initiative 1
        Stack::new(
            "imm2".to_string(),
            72,
            5294,
            vec!["slashing".to_string()],
            vec!["radiation".to_string(), "cold".to_string()],
            639,
            "fire".to_string(),
            1,
        ),
        // 4713 units each with 6987 hit points (weak to radiation) with an attack that does 12 slashing damage at initiative 2
        Stack::new(
            "imm3".to_string(),
            4713,
            6987,
            vec!["radiation".to_string()],
            vec![],
            12,
            "slashing".to_string(),
            2,
        ),
        // 623 units each with 9745 hit points with an attack that does 137 cold damage at initiative 6
        Stack::new(
            "imm4".to_string(),
            623,
            9745,
            vec![],
            vec![],
            137,
            "cold".to_string(),
            6,
        ),
        // 1412 units each with 9165 hit points with an attack that does 52 bludgeoning damage at initiative 3
        Stack::new(
            "imm5".to_string(),
            1412,
            9165,
            vec![],
            vec![],
            52,
            "bludgeoning".to_string(),
            3,
        ),
        // 2042 units each with 7230 hit points (immune to cold, radiation) with an attack that does 25 bludgeoning damage at initiative 15
        Stack::new(
            "imm6".to_string(),
            2042,
            7230,
            vec![],
            vec!["cold".to_string(), "radiation".to_string()],
            25,
            "bludgeoning".to_string(),
            15,
        ),
        // 209 units each with 9954 hit points with an attack that does 384 cold damage at initiative 17
        Stack::new(
            "imm7".to_string(),
            209,
            9954,
            vec![],
            vec![],
            384,
            "cold".to_string(),
            17,
        ),
        // 33 units each with 6495 hit points (weak to fire) with an attack that does 1756 fire damage at initiative 7
        Stack::new(
            "imm8".to_string(),
            33,
            6495,
            vec!["fire".to_string()],
            vec![],
            1756,
            "fire".to_string(),
            7,
        ),
        // 242 units each with 6650 hit points (immune to radiation, fire) with an attack that does 239 bludgeoning damage at initiative 12
        Stack::new(
            "imm9".to_string(),
            242,
            6650,
            vec![],
            vec!["fire".to_string(), "radiation".to_string()],
            239,
            "bludgeoning".to_string(),
            12,
        ),
        // 4701 units each with 7384 hit points (immune to cold) with an attack that does 14 fire damage at initiative 9
        Stack::new(
            "imm10".to_string(),
            4701,
            7384,
            vec![],
            vec!["cold".to_string()],
            14,
            "fire".to_string(),
            9,
        ),
    ];

    let mut infection = vec![
        //4154 units each with 21287 hit points (immune to fire, slashing, cold, radiation) with an attack that does 9 fire damage at initiative 5
        Stack::new(
            "inf1".to_string(),
            4154,
            21287,
            vec![],
            vec![
                "fire".to_string(),
                "slashing".to_string(),
                "cold".to_string(),
                "radiation".to_string(),
            ],
            9,
            "fire".to_string(),
            5,
        ),
        // 2091 units each with 5531 hit points (immune to slashing) with an attack that does 5 fire damage at initiative 13
        Stack::new(
            "inf2".to_string(),
            2091,
            5531,
            vec![],
            vec!["slashing".to_string()],
            5,
            "fire".to_string(),
            13,
        ),
        // 2237 units each with 24000 hit points with an attack that does 20 fire damage at initiative 16
        Stack::new(
            "inf3".to_string(),
            2237,
            24000,
            vec![],
            vec![],
            20,
            "fire".to_string(),
            16,
        ),
        // 149 units each with 31282 hit points (weak to radiation, cold) with an attack that does 329 radiation damage at initiative 8
        Stack::new(
            "inf4".to_string(),
            149,
            31282,
            vec!["radiation".to_string(), "cold".to_string()],
            vec![],
            329,
            "radiation".to_string(),
            8,
        ),
        // 649 units each with 39642 hit points with an attack that does 108 cold damage at initiative 18
        Stack::new(
            "inf5".to_string(),
            649,
            39642,
            vec![],
            vec![],
            108,
            "cold".to_string(),
            18,
        ),
        // 108 units each with 35626 hit points (immune to radiation; weak to slashing) with an attack that does 519 cold damage at initiative 4
        Stack::new(
            "inf6".to_string(),
            108,
            35626,
            vec!["slashing".to_string()],
            vec!["radiation".to_string()],
            519,
            "cold".to_string(),
            4,
        ),
        // 1194 units each with 37567 hit points (weak to fire, radiation) with an attack that does 59 radiation damage at initiative 19
        Stack::new(
            "inf7".to_string(),
            1194,
            37567,
            vec!["fire".to_string(), "radiation".to_string()],
            vec![],
            59,
            "radiation".to_string(),
            19,
        ),
        // 2849 units each with 37603 hit points (immune to cold) with an attack that does 26 bludgeoning damage at initiative 10
        Stack::new(
            "inf8".to_string(),
            2849,
            37603,
            vec![],
            vec!["cold".to_string()],
            26,
            "bludgeoning".to_string(),
            10,
        ),
        // 451 units each with 35892 hit points (weak to slashing; immune to cold) with an attack that does 133 fire damage at initiative 14
        Stack::new(
            "inf9".to_string(),
            451,
            35892,
            vec!["slashing".to_string()],
            vec!["cold".to_string()],
            133,
            "fire".to_string(),
            14,
        ),
        // 3232 units each with 27332 hit points (weak to fire) with an attack that does 14 cold damage at initiative 11
        Stack::new(
            "inf".to_string(),
            3232,
            27332,
            vec!["fire".to_string()],
            vec![],
            14,
            "cold".to_string(),
            11,
        ),
    ];

    win_battle(&mut immune_system, &mut infection).to_string()
}

#[derive(Eq, PartialEq, Hash)]
struct Stack {
    name: String,
    units: usize,
    hp: usize,
    weakness: Vec<String>,
    immunity: Vec<String>,
    attack: usize,
    attack_type: String,
    init: usize,
}

impl Clone for Stack {
    fn clone(&self) -> Stack {
        Stack {
            name: String::from(self.name.to_string()),
            units: self.units,
            hp: self.hp,
            weakness: self.weakness.iter().cloned().collect(),
            immunity: self.immunity.iter().cloned().collect(),
            attack: self.attack,
            attack_type: String::from(self.attack_type.to_string()),
            init: self.init,
        }
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{},#{},hp:{}}}", self.name, self.units, self.hp)
    }
}

impl Stack {
    fn new(
        name: String,
        units: usize,
        hp: usize,
        weakness: Vec<String>,
        immunity: Vec<String>,
        attack: usize,
        attack_type: String,
        init: usize,
    ) -> Stack {
        Stack {
            name,
            units,
            hp,
            weakness,
            immunity,
            attack,
            attack_type,
            init,
        }
    }
}

fn win_battle(immune_system: &mut Vec<Stack>, infection: &mut Vec<Stack>) -> usize {
    let mut boost = 30;
    loop {
        boost += 1;
        let mut sample_immune: Vec<Stack> = immune_system
            .iter()
            .cloned()
            .map(|defender| {
                Stack::new(
                    defender.name.to_string(),
                    defender.units,
                    defender.hp,
                    defender.weakness,
                    defender.immunity,
                    defender.attack + boost,
                    defender.attack_type.to_string(),
                    defender.init,
                )
            }).collect();
        let mut sample_infection = infection.iter().cloned().collect();
        let result = battle(&mut sample_immune, &mut sample_infection);
        if result.1 {
            return result.0;
        }
    }
}

fn battle(immune_system: &mut Vec<Stack>, infection: &mut Vec<Stack>) -> (usize, bool) {
    loop {
        let mut targeting: Vec<Stack> = immune_system
            .iter()
            .chain(infection.iter())
            .cloned()
            .collect();
        targeting.sort_unstable_by(|left, right| {
            if (left.attack * left.units) == (right.attack * right.units) {
                right.init.cmp(&left.init)
            } else {
                (right.attack * right.units).cmp(&(left.attack * left.units))
            }
        });
        // println!("targeting order round {}:{:?}", counter, targeting);

        let mut target_pairs = vec![];
        let mut claimed_targets: Vec<Stack> = vec![];
        let mut someone_died = false;

        for targeter in targeting.iter() {
            let mut best_damage = -1;
            let mut best_target: Stack =
                Stack::new("".to_string(), 0, 0, vec![], vec![], 0, "".to_string(), 0);

            let targets;
            if immune_system.contains(targeter) {
                targets = infection.clone();
            } else {
                targets = immune_system.clone();
            }
            // println!("{:?} VS {:?}", targets, claimed_targets);
            for target in targets.iter().filter(|x| !claimed_targets.contains(x)) {
                if target.immunity.contains(&targeter.attack_type) {
                    continue;
                }
                if target.weakness.contains(&targeter.attack_type) {
                    if best_damage == 2 {
                        if target.attack * target.units > best_target.attack * best_target.units {
                            best_target = target.clone();
                        } else if (target.attack * target.units
                            == best_target.attack * best_target.units)
                            && target.init > best_target.init
                        {
                            best_target = target.clone();
                        }
                    } else {
                        best_damage = 2;
                        best_target = target.clone();
                    }
                } else {
                    if best_damage == 2 {
                        continue;
                    }
                    if best_damage == 1 {
                        if target.attack * target.units > best_target.attack * best_target.units {
                            best_target = target.clone();
                        } else if (target.attack * target.units
                            == best_target.attack * best_target.units)
                            && target.init > best_target.init
                        {
                            best_target = target.clone();
                        }
                    } else {
                        best_damage = 1;
                        best_target = target.clone();
                    }
                }
            }
            if best_damage > 0 {
                target_pairs.push((targeter, best_target.clone()));
                claimed_targets.push(best_target);
            }
        }

        target_pairs.sort_unstable_by(|left, right| right.0.init.cmp(&left.0.init));
        // println!("picked targets:{:?}", target_pairs);
        let mut before_after: Vec<(Stack, Stack)> = vec![];
        for (attacker, defender) in target_pairs {
            let updated_attacker: Vec<Stack> = before_after
                .iter()
                .filter(|x| x.0 == *attacker)
                .map(|x| x.1.clone())
                .collect();
            let real_attacker;

            if updated_attacker.len() == 1 {
                real_attacker = updated_attacker[0].clone();
            } else {
                real_attacker = attacker.clone();
            }
            if real_attacker.units == 0 {
                // println!("skipping {} since now dead", attacker.name);
                continue;
            }

            let dmg_mult;
            if defender.weakness.contains(&real_attacker.attack_type) {
                // println!("weak!");
                dmg_mult = 2;
            } else {
                dmg_mult = 1;
            }

            let result;
            let dmg = real_attacker.units * real_attacker.attack * dmg_mult;
            let dead = dmg / defender.hp;
            if dead > 0 {
                someone_died = true;
            }
            if dead <= defender.units {
                // println!(
                //     "{} attacked {} killing {}",
                //     attacker.name, defender.name, dead
                // );
                let defender2 = defender.clone();
                result = Stack::new(
                    defender2.name.to_string(),
                    defender2.units - dead,
                    defender2.hp,
                    defender2.weakness,
                    defender2.immunity,
                    defender2.attack,
                    defender2.attack_type.to_string(),
                    defender2.init,
                );
            } else {
                // println!(
                //     "{} attacked {} killing all {}",
                //     attacker.name, defender.name, defender.units
                // );
                let defender2 = defender.clone();
                result = Stack::new(
                    defender2.name.to_string(),
                    0,
                    defender2.hp,
                    defender2.weakness,
                    defender2.immunity,
                    defender2.attack,
                    defender2.attack_type.to_string(),
                    defender2.init,
                );
            }
            before_after.push((defender, result));
        }

        for (old, update) in before_after {
            if immune_system.contains(&old) {
                immune_system.retain(|x| *x != old);
                if update.units > 0 {
                    immune_system.push(update);
                }
            } else {
                infection.retain(|x| *x != old);;
                if update.units > 0 {
                    infection.push(update);
                }
            }
        }
        if immune_system.len() == 0 {
            return (infection.iter().map(|x| x.units).sum(), false);
        } else if infection.len() == 0 {
            return (immune_system.iter().map(|x| x.units).sum(), true);
        }
        if !someone_died {
            return (0, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_small_sample24() {
        let mut immune_system = vec![
            Stack::new(
                "i1".to_string(),
                17,
                5390,
                vec!["radiation".to_string(), "bludgeoning".to_string()],
                vec![],
                4507,
                "fire".to_string(),
                2,
            ),
            Stack::new(
                "i2".to_string(),
                989,
                1274,
                vec!["bludgeoning".to_string(), "slashing".to_string()],
                vec!["fire".to_string()],
                25,
                "slashing".to_string(),
                3,
            ),
        ];

        let mut infection = vec![
            Stack::new(
                "f1".to_string(),
                801,
                4706,
                vec!["radiation".to_string()],
                vec![],
                116,
                "bludgeoning".to_string(),
                1,
            ),
            Stack::new(
                "f2".to_string(),
                4485,
                2961,
                vec!["fire".to_string(), "cold".to_string()],
                vec!["radiation".to_string()],
                12,
                "slashing".to_string(),
                4,
            ),
        ];

        let actual = battle(&mut immune_system, &mut infection).0;

        assert_eq!(actual, 5216)
    }

    #[test]
    fn should_not_let_dead_attack() {
        let mut immune_system = vec![Stack::new(
            "i1".to_string(),
            2,
            5,
            vec!["radiation".to_string(), "bludgeoning".to_string()],
            vec![],
            100,
            "fire".to_string(),
            2,
        )];

        let mut infection = vec![Stack::new(
            "f1".to_string(),
            1,
            5,
            vec!["radiation".to_string()],
            vec![],
            100,
            "bludgeoning".to_string(),
            1,
        )];

        let actual = battle(&mut immune_system, &mut infection).0;

        assert_eq!(actual, 2)
    }

    #[test]
    fn should_tiebreak_targeting() {
        let mut immune_system = vec![
            Stack::new(
                "imm1".to_string(),
                2,
                5,
                vec!["radiation".to_string(), "bludgeoning".to_string()],
                vec![],
                100,
                "radiation".to_string(),
                1,
            ),
            Stack::new(
                "imm2".to_string(),
                2,
                5,
                vec!["radiation".to_string(), "bludgeoning".to_string()],
                vec![],
                100,
                "fire".to_string(),
                3,
            ),
        ];

        let mut infection = vec![Stack::new(
            "f1".to_string(),
            1,
            5,
            vec![],
            vec!["radiation".to_string()],
            100,
            "bludgeoning".to_string(),
            2,
        )];

        let actual = battle(&mut immune_system, &mut infection).0;

        assert_eq!(actual, 4)
    }

    #[test]
    fn should_tiebreak_targeting_again() {
        let mut immune_system = vec![
            Stack::new(
                "imm1".to_string(),
                1,
                5,
                vec![],
                vec![],
                100,
                "radiation".to_string(),
                1,
            ),
            Stack::new(
                "imm2".to_string(),
                2,
                5,
                vec![],
                vec![],
                50,
                "fire".to_string(),
                3,
            ),
        ];

        let mut infection = vec![Stack::new(
            "f1".to_string(),
            1,
            5,
            vec![],
            vec!["radiation".to_string()],
            100,
            "bludgeoning".to_string(),
            4,
        )];

        let actual = battle(&mut immune_system, &mut infection).0;

        assert_eq!(actual, 1)
    }

}
