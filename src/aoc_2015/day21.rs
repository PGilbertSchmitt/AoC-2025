use std::i64;

use fxhash::FxHashSet;

#[derive(Hash, Eq, PartialEq)]
struct Item {
    cost: i64,
    attack: i64,
    defense: i64,
}

// (cost, damage)
const WEAPON_STORE: [Item; 5] = [
    Item {
        cost: 8,
        attack: 4,
        defense: 0,
    },
    Item {
        cost: 10,
        attack: 5,
        defense: 0,
    },
    Item {
        cost: 25,
        attack: 6,
        defense: 0,
    },
    Item {
        cost: 40,
        attack: 7,
        defense: 0,
    },
    Item {
        cost: 74,
        attack: 8,
        defense: 0,
    },
];

// (cost, armor)
const ARMOR_STORE: [Item; 6] = [
    Item {
        cost: 0,
        attack: 0,
        defense: 0,
    }, // The NONE option
    Item {
        cost: 13,
        attack: 0,
        defense: 1,
    },
    Item {
        cost: 31,
        attack: 0,
        defense: 2,
    },
    Item {
        cost: 53,
        attack: 0,
        defense: 3,
    },
    Item {
        cost: 75,
        attack: 0,
        defense: 4,
    },
    Item {
        cost: 102,
        attack: 0,
        defense: 5,
    },
];

// (cost, damage, armor)
const RING_STORE: [Item; 7] = [
    Item {
        cost: 0,
        attack: 0,
        defense: 0,
    }, // The NONE option
    Item {
        cost: 25,
        attack: 1,
        defense: 0,
    },
    Item {
        cost: 50,
        attack: 2,
        defense: 0,
    },
    Item {
        cost: 100,
        attack: 3,
        defense: 0,
    },
    Item {
        cost: 20,
        attack: 0,
        defense: 1,
    },
    Item {
        cost: 40,
        attack: 0,
        defense: 2,
    },
    Item {
        cost: 80,
        attack: 0,
        defense: 3,
    },
];

struct Build {
    health: i64,
    damage: i64,
    armor: i64,
}

impl Build {
    fn augment(
        &self,
        &Item {
            attack: damage,
            defense: armor,
            ..
        }: &Item,
    ) -> Self {
        Self {
            health: self.health,
            damage: self.damage + damage,
            armor: self.armor + armor,
        }
    }

    fn can_beat(&self, other: &Build) -> bool {
        let applied_self_damage = 1.max(self.damage - other.armor);
        let turns_to_beat_other =
            (other.health / applied_self_damage) + (other.health % applied_self_damage).min(1);

        let applied_boss_damage = 1.max(other.damage - self.armor);
        let turns_to_beat_self =
            (self.health / applied_boss_damage) + (self.health % applied_boss_damage).min(1);

        // Since the player goes first, if they would take the same number of turns to beat the boss, they will get the last hit before the boss does
        turns_to_beat_other <= turns_to_beat_self
    }
}

// Vec of all combinations of (cost, damage, armor) for 1 weapon, 0-1 armor, and 0-2 rings
// I could be more optimal, but this is truly such a small amount of total work that I
// prefer the simpler, more elegant, slightly less optimal solution.
fn all_combos() -> Vec<Item> {
    let mut combos = FxHashSet::default();

    for weapon in WEAPON_STORE.iter() {
        for armor in ARMOR_STORE.iter() {
            for ring1 in RING_STORE.iter() {
                for ring2 in RING_STORE.iter() {
                    if ring1 != ring2 {
                        combos.insert(Item {
                            cost: weapon.cost + armor.cost + ring1.cost + ring2.cost,
                            attack: weapon.attack + ring1.attack + ring2.attack,
                            defense: armor.defense + ring1.defense + ring2.defense,
                        });
                    }
                }
            }
        }
    }

    combos.into_iter().collect()
}

fn find_extreme_builds() -> (i64, i64) {
    let base_player = Build {
        health: 100,
        damage: 0,
        armor: 0,
    };

    // The AofC day 21 input
    let boss = Build {
        health: 103,
        damage: 9,
        armor: 2,
    };

    let mut lowest_cost_winner = i64::MAX;
    let mut highest_cost_loser = 0;
    for item in all_combos() {
        if base_player.augment(&item).can_beat(&boss) {
            lowest_cost_winner = lowest_cost_winner.min(item.cost);
        } else {
            highest_cost_loser = highest_cost_loser.max(item.cost);
        }
    }

    (lowest_cost_winner, highest_cost_loser)
}

#[test]
fn both_parts() {
    assert_eq!((121, 201), find_extreme_builds());
}
