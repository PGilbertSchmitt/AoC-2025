use std::ops::Add;

struct Behavior {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Behavior {
    fn total(&self, count: i32) -> Behavior {
        Behavior {
            capacity: self.capacity * count,
            durability: self.durability * count,
            flavor: self.flavor * count,
            texture: self.texture * count,
            calories: self.calories * count,
        }
    }

    fn to_score(&self) -> i32 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}

impl Add for Behavior {
    type Output = Behavior;

    fn add(self, rhs: Self) -> Self::Output {
        Behavior {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

/* It's just 4 lines, so it's definitely faster to encode direction than to parse it.
 * The input:

Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8
Butterscotch: capacity -1, durability 0, flavor 5, texture 0, calories 6
Sugar: capacity 0, durability 0, flavor -2, texture 2, calories 1

 */

const FROSTING: Behavior = Behavior {
    capacity: 4,
    durability: -2,
    flavor: 0,
    texture: 0,
    calories: 5,
};

const CANDY: Behavior = Behavior {
    capacity: 0,
    durability: 5,
    flavor: -1,
    texture: 0,
    calories: 8,
};

const BUTTERSCOTCH: Behavior = Behavior {
    capacity: -1,
    durability: 0,
    flavor: 5,
    texture: 0,
    calories: 6,
};

const SUGAR: Behavior = Behavior {
    capacity: 0,
    durability: 0,
    flavor: -2,
    texture: 2,
    calories: 1,
};

const LIMIT: i32 = 101;

#[test]
fn part_1() {
    let mut high_score = 0;
    for a in 0..LIMIT {
        for b in 0..(LIMIT - a) {
            for c in 0..(LIMIT - a - b) {
                let d = LIMIT - a - b - c - 1;

                let cur_score =
                    (FROSTING.total(a) + CANDY.total(b) + BUTTERSCOTCH.total(c) + SUGAR.total(d))
                        .to_score();

                high_score = high_score.max(cur_score);
            }
        }
    }

    assert_eq!(18965440, high_score);
}

#[test]
fn part_2() {
    let mut high_score = 0;
    for a in 0..LIMIT {
        for b in 0..(LIMIT - a) {
            for c in 0..(LIMIT - a - b) {
                let d = LIMIT - a - b - c - 1;

                let cur_behavior =
                    FROSTING.total(a) + CANDY.total(b) + BUTTERSCOTCH.total(c) + SUGAR.total(d);

                if cur_behavior.calories == 500 {
                    high_score = high_score.max(cur_behavior.to_score());
                }
            }
        }
    }

    assert_eq!(15862900, high_score);
}
