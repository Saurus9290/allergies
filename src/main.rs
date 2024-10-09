use self:: Allergen::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];
pub struct Allergies {
    allergens: u32,
}
impl Allergies {
    pub fn new(n: u32) -> Allergies {
        Allergies { allergens: n}
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.allergens & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter().filter(|a| self.is_allergic_to(a)).cloned().collect()
    }
}

fn main() {
    // Create a new Allergies instance for a person with an allergy score of 5
    let my_allergies = Allergies::new(5);

    // Check if the person is allergic to Eggs and Shellfish
    println!(
        "Allergic to Eggs: {}",
        my_allergies.is_allergic_to(&Allergen::Eggs)
    );
    println!(
        "Allergic to Shellfish: {}",
        my_allergies.is_allergic_to(&Allergen::Shellfish)
    );

    // List all the allergens the person is allergic to
    let allergies = my_allergies.allergies();
    println!("Allergies: {:?}", allergies);
}