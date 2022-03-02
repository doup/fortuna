use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
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

#[derive(Debug)]
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

pub struct StatsRes(pub Stats);

#[derive(Debug)]
pub struct Stats {
    pub color: SkinColor,
    pub has_mental_illness: bool,
    pub has_supportive_family: bool,
    pub intelligence: Intelligence,
    pub is_male: bool,
    pub strength: Strength,
    pub wealth: Wealth,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            color: rand::random(),
            has_mental_illness: rand::random(),
            has_supportive_family: rand::random(),
            intelligence: rand::random(),
            is_male: rand::random(),
            strength: rand::random(),
            wealth: rand::random(),
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

        let mental_health = match self.has_mental_illness {
            true => String::from(""),
            false => String::from("mentally healthy, "),
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
        let stats = Stats {
            color: SkinColor::Light,
            has_mental_illness: false,
            has_supportive_family: true,
            intelligence: Intelligence::Smart,
            is_male: true,
            strength: Strength::Strong,
            wealth: Wealth::Rich,
        };

        assert_eq!("You're a man born to a rich supportive family. You're mentally healthy, physically strong and fairly smart.", stats.get_description());

        let stats = Stats {
            color: SkinColor::Light,
            has_mental_illness: true,
            has_supportive_family: false,
            intelligence: Intelligence::Dumb,
            is_male: false,
            strength: Strength::Weak,
            wealth: Wealth::Poor,
        };

        assert_eq!("You're a woman born to a poor unstructured family. You're not very strong and not very smart.", stats.get_description());

        let stats = Stats {
            color: SkinColor::Light,
            has_mental_illness: false,
            has_supportive_family: true,
            intelligence: Intelligence::Smart,
            is_male: false,
            strength: Strength::Weak,
            wealth: Wealth::MiddleClass,
        };

        assert_eq!("You're a woman born to a middle-class supportive family. You're mentally healthy, not very strong but you're fairly smart.", stats.get_description());
    }
}
