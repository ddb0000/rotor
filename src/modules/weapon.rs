use rand::{Rng, distributions::{Distribution, Standard}};

pub struct Weapon {
    pub name: String,
    pub damage: i32,
    pub range: f32,
    pub effect: WeaponEffect,
}

// Define different types of weapon effects here
pub enum WeaponEffect {
    Fire,
    Ice,
    Poison,
}

// Randomly generate a weapon effect
impl Distribution<WeaponEffect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WeaponEffect {
        match rng.gen_range(0..3) {
            0 => WeaponEffect::Fire,
            1 => WeaponEffect::Ice,
            _ => WeaponEffect::Poison,
        }
    }
}

impl Weapon {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        // Generate random properties for the weapon
        let damage = rng.gen_range(5..=15); // Example range, adjust as needed
        let range = rng.gen_range(1.0..=5.0); // Example range, adjust as needed
        let effect = rng.gen(); // Randomly generate a weapon effect

        // Add logic here to generate a name based on the weapon's properties or effect
        let name = format!("Weapon of {:?}", effect);

        Weapon {
            name,
            damage,
            range,
            effect,
        }
    }

    // ... methods for weapon usage, interactions, etc. ...
}

