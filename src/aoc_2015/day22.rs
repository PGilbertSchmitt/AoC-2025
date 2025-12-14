use std::{collections::BinaryHeap, i32};

#[derive(PartialEq, Eq)]
struct GameState {
    // Player
    player_health: i32,
    player_shield: i32,
    mana: i32,
    shield_remaining: u32,
    poison_remaining: u32,
    recharge_remaining: u32,
    total_mana_spend: i32,

    // Boss
    boss_health: i32,
}

#[derive(PartialEq, Eq)]
enum Action {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const BOSS_DAMAGE: i32 = 9;

const MAGIC_MISSILE_MANA: i32 = 53;
const MAGIC_MISSILE_DAMAGE: i32 = 4;

const DRAIN_MANA: i32 = 73;
const DRAIN_AMOUNT: i32 = 2;

const SHIELD_MANA: i32 = 113;
const SHIELD_TURNS: u32 = 6;
const SHIELD_AMOUNT: i32 = 7;

const POISON_MANA: i32 = 173;
const POISON_TURNS: u32 = 6;
const POISON_DAMAGE: i32 = 3;

const RECHARGE_MANA: i32 = 229;
const RECHARGE_TURNS: u32 = 5;
const RECHARGE_AMOUNT: i32 = 101;

impl GameState {
    fn base_state() -> Self {
        Self {
            player_health: 50,
            player_shield: 0,
            mana: 500,
            shield_remaining: 0,
            poison_remaining: 0,
            recharge_remaining: 0,
            boss_health: 51,
            total_mana_spend: 0,
        }
    }

    fn available_actions(&self) -> Vec<Action> {
        let mut actions = Vec::with_capacity(5);
        if self.mana >= MAGIC_MISSILE_MANA {
            actions.push(Action::MagicMissile);

            if self.mana >= DRAIN_MANA {
                actions.push(Action::Drain);

                if self.mana >= SHIELD_MANA && self.shield_remaining == 0 {
                    actions.push(Action::Shield);
                }

                if self.mana >= POISON_MANA && self.poison_remaining == 0 {
                    actions.push(Action::Poison);
                }

                if self.mana >= RECHARGE_MANA && self.recharge_remaining == 0 {
                    actions.push(Action::Recharge)
                }
            }
        }

        actions
    }

    // Returns true if the game is still going, false if the player loses
    fn apply_effects(&mut self, hard: bool) -> bool {
        if hard {
            if self.player_health == 1 {
                return false;
            } else {
                self.player_health -= 1;
            }
        }

        if self.shield_remaining > 0 {
            self.shield_remaining -= 1;
            self.player_shield = SHIELD_AMOUNT;
        } else {
            self.player_shield = 0;
        }

        if self.poison_remaining > 0 {
            self.poison_remaining -= 1;
            self.boss_health -= POISON_DAMAGE;
        }

        if self.recharge_remaining > 0 {
            self.recharge_remaining -= 1;
            self.mana += RECHARGE_AMOUNT;
        }

        true
    }

    // Returns true if the game is still going, false if the player loses
    fn apply_boss_attack(&mut self) -> bool {
        self.player_health -= BOSS_DAMAGE - self.player_shield;
        self.player_health > 0
    }

    fn apply_player_turn(&self, action: Action) -> Self {
        let (player_damage, health_gained, mana_used) = match action {
            Action::MagicMissile => (MAGIC_MISSILE_DAMAGE, 0, MAGIC_MISSILE_MANA),
            Action::Drain => (DRAIN_AMOUNT, DRAIN_AMOUNT, DRAIN_MANA),
            Action::Shield => (0, 0, SHIELD_MANA),
            Action::Poison => (0, 0, POISON_MANA),
            Action::Recharge => (0, 0, RECHARGE_MANA),
        };

        Self {
            player_health: self.player_health + health_gained,
            player_shield: self.player_shield,
            mana: self.mana - mana_used,
            shield_remaining: if action == Action::Shield {
                SHIELD_TURNS
            } else {
                self.shield_remaining
            },
            poison_remaining: if action == Action::Poison {
                POISON_TURNS
            } else {
                self.poison_remaining
            },
            recharge_remaining: if action == Action::Recharge {
                RECHARGE_TURNS
            } else {
                self.recharge_remaining
            },
            boss_health: self.boss_health - player_damage,
            total_mana_spend: self.total_mana_spend + mana_used,
        }
    }

    fn is_win(&self) -> bool {
        self.boss_health <= 0
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.total_mana_spend.partial_cmp(&self.total_mana_spend)
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_mana_spend.cmp(&self.total_mana_spend)
    }
}

fn find_lowest_mana_win(hard: bool) -> i32 {
    // Store state from the moment after a player has used a turn.
    let mut game_states = BinaryHeap::<GameState>::new();
    let base_state = GameState::base_state();
    for first_action in base_state.available_actions() {
        let mut first_state = base_state.apply_player_turn(first_action);
        first_state.apply_effects(hard);
        game_states.push(first_state);
    }

    let mut lowest_mana_cost = i32::MAX;
    while let Some(mut game_state) = game_states.pop() {
        if game_state.total_mana_spend >= lowest_mana_cost {
            continue;
        }
        // Boss's turn
        game_state.apply_effects(false);
        if game_state.is_win() {
            lowest_mana_cost = lowest_mana_cost.min(game_state.total_mana_spend);
        } else if game_state.apply_boss_attack() {
            // Player's turn
            if game_state.apply_effects(hard) {
                for action in game_state.available_actions() {
                    let next_state = game_state.apply_player_turn(action);
                    if game_state.is_win() {
                        lowest_mana_cost = lowest_mana_cost.min(game_state.total_mana_spend);
                    } else {
                        game_states.push(next_state);
                    }
                }
            }
        }
    }

    lowest_mana_cost
}

#[test]
fn both_parts() {
    assert_eq!(900, find_lowest_mana_win(false));
    assert_eq!(1216, find_lowest_mana_win(true));
}
