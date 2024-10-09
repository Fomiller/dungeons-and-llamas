use rand::Rng;

pub struct Dice {
    sides: usize,
}

impl Dice {
    pub fn new(sides: usize) -> Dice {
        Self { sides }
    }
    pub fn roll(self) -> usize {
        rand::thread_rng().gen_range(1..self.sides)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let die = Dice::new(20);
        let result = die.roll();
        assert_eq!(result <= 20, true);
    }
}
