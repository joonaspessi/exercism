pub struct Allergies(u32);

// 0 EGGS (1)
// 1 PEANUTS (2)
// 2 SHELLFISH (4)
// 3 STRAWBERRIES (8)
// 4 TOMATOES (16)
// 5 CHOCOLATE (32)
// 6 POLLEN (64)
// 7 CATS (128)

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        println!("{:#010b}", score);
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = vec![];
        let score = format!("{:b}", self.0 % 256);
        for (index, allergy) in score.chars().rev().enumerate() {
            println!("{} {}", index, allergy);
            if allergy == '0' {
                continue;
            }
            match index {
                0 => allergies.push(Allergen::Eggs),
                1 => allergies.push(Allergen::Peanuts),
                2 => allergies.push(Allergen::Shellfish),
                3 => allergies.push(Allergen::Strawberries),
                4 => allergies.push(Allergen::Tomatoes),
                5 => allergies.push(Allergen::Chocolate),
                6 => allergies.push(Allergen::Pollen),
                7 => allergies.push(Allergen::Cats),
                _ => {}
            }
        }
        allergies
    }
}
