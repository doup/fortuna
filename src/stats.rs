use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::game::{GRAVITY, PLAYER_HEIGHT};

// RUN
const RUN_TOP_SPEED_STRONG: f32 = 180.0;
const RUN_TOP_SPEED_WEAK: f32 = 150.0;
const RUN_TOP_SPEED_DEPRESSED: f32 = 80.0;
const RUN_TOP_SPEED_TIME: f32 = 100.0; // Time in ms to get to top speed
const RUN_STOP_TIME: f32 = 50.0; // Time in ms to stop
const RUN_TOP_SPEED_RATE_STRONG: f32 = RUN_TOP_SPEED_STRONG / (RUN_TOP_SPEED_TIME / 1000.0);
const RUN_STOP_RATE_STRONG: f32 = RUN_TOP_SPEED_STRONG / (RUN_STOP_TIME / 1000.0);
const RUN_TOP_SPEED_RATE_WEAK: f32 = RUN_TOP_SPEED_WEAK / (RUN_TOP_SPEED_TIME / 1000.0);
const RUN_STOP_RATE_WEAK: f32 = RUN_TOP_SPEED_WEAK / (RUN_STOP_TIME / 1000.0);
const RUN_TOP_SPEED_RATE_DEPRESSED: f32 = RUN_TOP_SPEED_DEPRESSED / (RUN_TOP_SPEED_TIME / 1000.0);
const RUN_STOP_RATE_DEPRESSED: f32 = RUN_TOP_SPEED_DEPRESSED / (RUN_STOP_TIME / 1000.0);

// JUMP
const JUMP_HEIGHT_STRONG: f32 = 4.0; // Height in "Player Heights"
const JUMP_HEIGHT_STRONG_PX: f32 = (JUMP_HEIGHT_STRONG - 1.0) * PLAYER_HEIGHT;
const JUMP_HEIGHT_WEAK: f32 = 3.0; // Height in "Player Heights"
const JUMP_HEIGHT_WEAK_PX: f32 = (JUMP_HEIGHT_WEAK - 1.0) * PLAYER_HEIGHT;
const JUMP_HEIGHT_DEPRESSED: f32 = 2.0; // Height in "Player Heights"
const JUMP_HEIGHT_DEPRESSED_PX: f32 = (JUMP_HEIGHT_DEPRESSED - 1.0) * PLAYER_HEIGHT;

// DEPRESSIVE STATE
const MIN_DEPRE_CHANCE: f64 = 0.15;
const MAX_DEPRE_CHANCE: f64 = 0.60;

#[derive(Debug, PartialEq)]
pub enum Wealth {
    Poor,
    MiddleClass,
    Rich,
}

impl Distribution<Wealth> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wealth {
        match rng.gen_range(0..=2) {
            0 => Wealth::Poor,
            1 => Wealth::MiddleClass,
            _ => Wealth::Rich,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MentalHealth {
    Healthy,
    Depressive,
    Psychotic,
}

impl Distribution<MentalHealth> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MentalHealth {
        match rng.gen_range(0..=2) {
            0 => MentalHealth::Healthy,
            1 => MentalHealth::Depressive,
            _ => MentalHealth::Psychotic,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Strength {
    Weak,
    Strong,
}

impl Distribution<Strength> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Strength {
        match rng.gen_range(0..=1) {
            0 => Strength::Weak,
            _ => Strength::Strong,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SkinColor {
    Light,
    Medium,
    Dark,
}

impl Distribution<SkinColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinColor {
        match rng.gen_range(0..=2) {
            0 => SkinColor::Light,
            1 => SkinColor::Medium,
            _ => SkinColor::Dark,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Intelligence {
    Dumb,
    Smart,
}

impl Distribution<Intelligence> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Intelligence {
        match rng.gen_range(0..=1) {
            0 => Intelligence::Dumb,
            _ => Intelligence::Smart,
        }
    }
}

pub struct StatsRes {
    pub value: Stats,
}

#[derive(Debug)]
pub struct Stats {
    pub color: SkinColor,
    pub mental_health: MentalHealth,
    pub has_supportive_family: bool,
    pub intelligence: Intelligence,
    pub is_male: bool,
    pub strength: Strength,
    pub wealth: Wealth,
    // Computed
    pub is_depressive: bool,
    pub depre_chance: f64,
    pub can_skip_one_way_platforms: bool,
    pub top_speed: f32,
    pub top_speed_depressed: f32,
    pub top_speed_rate: f32,
    pub top_speed_rate_depressed: f32,
    pub stop_rate: f32,
    pub stop_rate_depressed: f32,
    pub jump_force: f32,
    pub jump_force_depressed: f32,
    pub lifes: i32,
}

impl Stats {
    pub fn new() -> Stats {
        Stats::from_config(
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        )
    }

    pub fn from_config(
        color: SkinColor,
        mental_health: MentalHealth,
        has_supportive_family: bool,
        intelligence: Intelligence,
        is_male: bool,
        strength: Strength,
        wealth: Wealth,
    ) -> Stats {
        let is_depressive = mental_health == MentalHealth::Depressive;
        let depre_chance = rand::thread_rng().gen_range(MIN_DEPRE_CHANCE..MAX_DEPRE_CHANCE);
        let can_skip_one_way_platforms = is_male;
        let top_speed_depressed = RUN_TOP_SPEED_DEPRESSED;
        let top_speed_rate_depressed = RUN_TOP_SPEED_RATE_DEPRESSED;
        let stop_rate_depressed = RUN_STOP_RATE_DEPRESSED;
        let jump_force_depressed = (-2.0 * GRAVITY * JUMP_HEIGHT_DEPRESSED_PX).sqrt();
        let top_speed;
        let top_speed_rate;
        let stop_rate;
        let jump_force;

        if strength == Strength::Strong {
            top_speed = RUN_TOP_SPEED_STRONG;
            top_speed_rate = RUN_TOP_SPEED_RATE_STRONG;
            stop_rate = RUN_STOP_RATE_STRONG;
            jump_force = (-2.0 * GRAVITY * JUMP_HEIGHT_STRONG_PX).sqrt();
        } else {
            top_speed = RUN_TOP_SPEED_WEAK;
            top_speed_rate = RUN_TOP_SPEED_RATE_WEAK;
            stop_rate = RUN_STOP_RATE_WEAK;
            jump_force = (-2.0 * GRAVITY * JUMP_HEIGHT_WEAK_PX).sqrt();
        }

        // Lifes
        let mut lifes = match wealth {
            Wealth::Rich => 3,
            Wealth::MiddleClass => 2,
            Wealth::Poor => 1,
        };

        if has_supportive_family && lifes < 3 {
            lifes += 1;
        }

        Stats {
            color,
            mental_health,
            has_supportive_family,
            intelligence,
            is_male,
            strength,
            wealth,
            // Computed
            can_skip_one_way_platforms,
            depre_chance,
            is_depressive,
            jump_force,
            jump_force_depressed,
            lifes,
            stop_rate,
            stop_rate_depressed,
            top_speed,
            top_speed_depressed,
            top_speed_rate,
            top_speed_rate_depressed,
        }
    }

    pub fn get_description(&self) -> String {
        let family = match self.has_supportive_family {
            true => String::from("supportive"),
            false => String::from("unstructured"),
        };

        let genre = match self.is_male {
            true => String::from("man"),
            false => String::from("woman"),
        };

        let mental_health = match self.mental_health {
            MentalHealth::Healthy => String::from("mentally healthy, "),
            _ => String::from(""),
        };

        let wealth = match self.wealth {
            Wealth::Poor => String::from("poor"),
            Wealth::MiddleClass => String::from("middle-class"),
            Wealth::Rich => String::from("rich"),
        };

        let strength = match self.strength {
            Strength::Weak => String::from("not very strong"),
            Strength::Strong => String::from("physically strong"),
        };

        let intelligence = match self.intelligence {
            Intelligence::Smart => String::from("fairly smart"),
            Intelligence::Dumb => String::from("not very smart"),
        };

        let and_or_but = if (self.strength == Strength::Strong
            && self.intelligence == Intelligence::Smart)
            || (self.strength == Strength::Weak && self.intelligence == Intelligence::Dumb)
        {
            String::from("and")
        } else {
            String::from("but you're")
        };

        format!("You're a {genre} born to a {wealth} {family} family. You're {mental_health}{strength} {and_or_but} {intelligence}.")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_description() {
        let stats = Stats::from_config(
            SkinColor::Light,
            MentalHealth::Healthy,
            true,
            Intelligence::Smart,
            true,
            Strength::Strong,
            Wealth::Rich,
        );

        assert_eq!("You're a man born to a rich supportive family. You're mentally healthy, physically strong and fairly smart.", stats.get_description());

        let stats = Stats::from_config(
            SkinColor::Light,
            MentalHealth::Depressive,
            false,
            Intelligence::Dumb,
            false,
            Strength::Weak,
            Wealth::Poor,
        );

        assert_eq!("You're a woman born to a poor unstructured family. You're not very strong and not very smart.", stats.get_description());

        let stats = Stats::from_config(
            SkinColor::Light,
            MentalHealth::Healthy,
            true,
            Intelligence::Smart,
            false,
            Strength::Weak,
            Wealth::MiddleClass,
        );

        assert_eq!("You're a woman born to a middle-class supportive family. You're mentally healthy, not very strong but you're fairly smart.", stats.get_description());
    }
}
