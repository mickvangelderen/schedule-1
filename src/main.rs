use strum::{EnumIs, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIs)]
enum Effect {
    AntiGravity,
    Athletic,
    Balding,
    BrightEyed,
    Calming,
    CalorieDense,
    Cyclopean,
    Disorienting,
    Electrifying,
    Energizing,
    Euphoric,
    Explosive,
    Focused,
    Foggy,
    Gingeritis,
    Glowing,
    Jennerising,
    Laxative,
    LongFaced,
    Munchies,
    Paranoia,
    Refreshing,
    Schizophrenia,
    Sedating,
    SeizureInducing,
    Shrinking,
    Slippery,
    Smelly,
    Sneaky,
    Spicy,
    ThoughtProvoking,
    Toxic,
    TropicThunder,
    Zombifying,
}

impl Effect {
    fn multiplier(&self) -> f64 {
        match self {
            Effect::AntiGravity => 0.54,
            Effect::Athletic => 0.32,
            Effect::Balding => 0.30,
            Effect::BrightEyed => 0.40,
            Effect::Calming => 0.10,
            Effect::CalorieDense => 0.28,
            Effect::Cyclopean => 0.56,
            Effect::Disorienting => 0.00,
            Effect::Electrifying => 0.50,
            Effect::Energizing => 0.22,
            Effect::Euphoric => 0.18,
            Effect::Explosive => 0.00,
            Effect::Focused => 0.16,
            Effect::Foggy => 0.36,
            Effect::Gingeritis => 0.20,
            Effect::Glowing => 0.48,
            Effect::Jennerising => 0.42,
            Effect::Laxative => 0.00,
            Effect::LongFaced => 0.52,
            Effect::Munchies => 0.12,
            Effect::Paranoia => 0.00,
            Effect::Refreshing => 0.14,
            Effect::Schizophrenia => 0.00,
            Effect::Sedating => 0.26,
            Effect::SeizureInducing => 0.00,
            Effect::Shrinking => 0.60,
            Effect::Slippery => 0.34,
            Effect::Smelly => 0.00,
            Effect::Sneaky => 0.24,
            Effect::Spicy => 0.38,
            Effect::ThoughtProvoking => 0.44,
            Effect::Toxic => 0.00,
            Effect::TropicThunder => 0.46,
            Effect::Zombifying => 0.58,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Seed {
    OGKush,
    SourDiesel,
    GreenCrack,
    GranddadyPurple,
}

impl Seed {
    fn product(&self) -> Product {
        match self {
            Seed::OGKush => Product {
                solid: Solid::Weed,
                effects: {
                    let mut effects = ProductEffects::default();
                    effects.add(Effect::Calming);
                    effects
                },
            },
            Seed::SourDiesel => Product {
                solid: Solid::Weed,
                effects: {
                    let mut effects = ProductEffects::default();
                    effects.add(Effect::Refreshing);
                    effects
                },
            },
            Seed::GreenCrack => Product {
                solid: Solid::Weed,
                effects: {
                    let mut effects = ProductEffects::default();
                    effects.add(Effect::Energizing);
                    effects
                },
            },
            Seed::GranddadyPurple => Product {
                solid: Solid::Weed,
                effects: {
                    let mut effects = ProductEffects::default();
                    effects.add(Effect::Sedating);
                    effects
                },
            },
        }
    }

    fn price(&self) -> f64 {
        match self {
            Seed::OGKush => 30.0,
            Seed::SourDiesel => 35.0,
            Seed::GreenCrack => 40.0,
            Seed::GranddadyPurple => 45.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Solid {
    Weed,
    Meth,
    Cocaine,
}

impl Solid {
    fn base_price(&self) -> f64 {
        match self {
            Solid::Weed => 35.0,
            Solid::Meth => 70.0,
            Solid::Cocaine => 150.0,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Substance {
    Cuke,
    FluMedicine,
    Gasoline,
    Donut,
    EnergyDrink,
    MouthWash,
    MotorOil,
    Banana,
    Chili,
    Iodine,
    Paracetamol,
    Viagra,
    HorseSemen,
    MegaBean,
    Addy,
    Battery,
}

impl Substance {
    fn price(&self) -> f64 {
        match self {
            Substance::Cuke => 2.0,
            Substance::FluMedicine => 5.0,
            Substance::Gasoline => 5.0,
            Substance::Donut => 3.0,
            Substance::EnergyDrink => 6.0,
            Substance::MouthWash => 4.0,
            Substance::MotorOil => 6.0,
            Substance::Banana => 2.0,
            Substance::Chili => 7.0,
            Substance::Iodine => 8.0,
            Substance::Paracetamol => 3.0,
            Substance::Viagra => 4.0,
            Substance::HorseSemen => 9.0,
            Substance::MegaBean => 7.0,
            Substance::Addy => 9.0,
            Substance::Battery => 8.0,
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    solid: Solid,
    effects: ProductEffects,
}

const MAX_EFFECTS: usize = 8;

#[derive(Debug, Clone, Default)]
struct ProductEffects([Option<Effect>; MAX_EFFECTS]);

impl ProductEffects {
    fn replace(&mut self, old: &Effect, new: Effect) {
        if let Some(index) = self.position(old) {
            if let Some(index) = self.position(&new) {
                self.remove(index);
            }
            self.0[index] = Some(new)
        }
    }

    fn add(&mut self, new: Effect) {
        if !self.has(&new) {
            if let Some(index) = self.0.iter().position(|e| e.is_none()) {
                self.0[index] = Some(new);
            }
        }
    }

    fn has(&self, effect: &Effect) -> bool {
        self.0.iter().any(|e| e.as_ref() == Some(effect))
    }

    fn position(&self, effect: &Effect) -> Option<usize> {
        self.0.iter().position(|e| e.as_ref() == Some(effect))
    }

    fn remove(&mut self, index: usize) {
        self.0[index] = None;
    }

    fn multiplier(&self) -> f64 {
        self.iter().map(Effect::multiplier).sum()
    }

    fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter().filter_map(|i| i.as_ref())
    }
}

impl Product {
    fn mix(&self, substance: &Substance) -> Product {
        let mut product = self.clone();

        // The implementation is order-sensitive.
        match substance {
            Substance::Cuke => {
                product.effects.replace(&Effect::Sneaky, Effect::Paranoia);
                product.effects.replace(&Effect::Foggy, Effect::Cyclopean);
                product.effects.replace(&Effect::Gingeritis, Effect::ThoughtProvoking);
                product.effects.replace(&Effect::Munchies, Effect::Athletic);
                product.effects.replace(&Effect::Slippery, Effect::Munchies);
                product.effects.replace(&Effect::Euphoric, Effect::Laxative);
                product.effects.replace(&Effect::Toxic, Effect::Euphoric);
                product.effects.add(Effect::Energizing);
            }
            Substance::FluMedicine => {
                product.effects.replace(&Effect::Calming, Effect::BrightEyed);
                product.effects.replace(&Effect::ThoughtProvoking, Effect::Gingeritis);
                product.effects.replace(&Effect::Cyclopean, Effect::Foggy);
                product.effects.replace(&Effect::Munchies, Effect::Slippery);
                product.effects.replace(&Effect::Athletic, Effect::Munchies);
                product.effects.replace(&Effect::Euphoric, Effect::Toxic);
                product.effects.replace(&Effect::Laxative, Effect::Euphoric);
                product.effects.replace(&Effect::Focused, Effect::Calming);
                product.effects.replace(&Effect::Electrifying, Effect::Refreshing);
                product.effects.replace(&Effect::Shrinking, Effect::Paranoia);
                product.effects.add(Effect::Sedating);
            }
            Substance::Gasoline => {
                product.effects.replace(&Effect::Euphoric, Effect::Spicy);
                product.effects.replace(&Effect::Energizing, Effect::Euphoric);
                product.effects.replace(&Effect::Gingeritis, Effect::Smelly);
                product.effects.replace(&Effect::Sneaky, Effect::TropicThunder);
                product.effects.replace(&Effect::Jennerising, Effect::Sneaky);
                product.effects.replace(&Effect::Munchies, Effect::Sedating);
                product.effects.replace(&Effect::Laxative, Effect::Foggy);
                product.effects.replace(&Effect::Disorienting, Effect::Glowing);
                product.effects.replace(&Effect::Paranoia, Effect::Calming);
                product.effects.replace(&Effect::Electrifying, Effect::Disorienting);
                product.effects.replace(&Effect::Shrinking, Effect::Focused);
                product.effects.add(Effect::Toxic);
            }
            Substance::Donut => {
                product.effects.replace(&Effect::CalorieDense, Effect::Explosive);
                product.effects.replace(&Effect::Balding, Effect::Sneaky);
                product.effects.replace(&Effect::AntiGravity, Effect::Slippery);
                product.effects.replace(&Effect::Jennerising, Effect::Gingeritis);
                product.effects.replace(&Effect::Focused, Effect::Euphoric);
                product.effects.replace(&Effect::Shrinking, Effect::Energizing);
                product.effects.add(Effect::CalorieDense);
            }
            Substance::EnergyDrink => {
                product.effects.replace(&Effect::Sedating, Effect::Munchies);
                product.effects.replace(&Effect::Euphoric, Effect::Energizing);
                product.effects.replace(&Effect::Spicy, Effect::Euphoric);
                product.effects.replace(&Effect::TropicThunder, Effect::Sneaky);
                product.effects.replace(&Effect::Foggy, Effect::Laxative);
                product.effects.replace(&Effect::Disorienting, Effect::Electrifying);
                product.effects.replace(&Effect::Glowing, Effect::Disorienting);
                product.effects.replace(&Effect::Schizophrenia, Effect::Balding);
                product.effects.replace(&Effect::Focused, Effect::Shrinking);
                product.effects.add(Effect::Athletic);
            }
            Substance::MouthWash => {
                product.effects.replace(&Effect::Calming, Effect::AntiGravity);
                product.effects.replace(&Effect::CalorieDense, Effect::Sneaky);
                product.effects.replace(&Effect::Explosive, Effect::Sedating);
                product.effects.replace(&Effect::Focused, Effect::Jennerising);
                product.effects.add(Effect::Balding);
            }
            Substance::MotorOil => {
                product.effects.replace(&Effect::Munchies, Effect::Schizophrenia);
                product.effects.replace(&Effect::Energizing, Effect::Munchies);
                product.effects.replace(&Effect::Foggy, Effect::Toxic);
                product.effects.replace(&Effect::Euphoric, Effect::Sedating);
                product.effects.replace(&Effect::Paranoia, Effect::AntiGravity);
                product.effects.add(Effect::Slippery);
            }
            Substance::Banana => {
                product.effects.replace(&Effect::Calming, Effect::Sneaky);
                product.effects.replace(&Effect::Smelly, Effect::AntiGravity);
                product.effects.replace(&Effect::Toxic, Effect::Smelly);
                product.effects.replace(&Effect::LongFaced, Effect::Refreshing);
                product.effects.replace(&Effect::Cyclopean, Effect::ThoughtProvoking);
                product.effects.replace(&Effect::Focused, Effect::SeizureInducing);
                product.effects.replace(&Effect::Disorienting, Effect::Focused);
                product.effects.replace(&Effect::Paranoia, Effect::Jennerising);
                product.effects.add(Effect::Gingeritis);
            }
            Substance::Chili => {
                product.effects.replace(&Effect::Athletic, Effect::Euphoric);
                product.effects.replace(&Effect::AntiGravity, Effect::TropicThunder);
                product.effects.replace(&Effect::Sneaky, Effect::BrightEyed);
                product.effects.replace(&Effect::Munchies, Effect::Toxic);
                product.effects.replace(&Effect::Laxative, Effect::LongFaced);
                product.effects.replace(&Effect::Shrinking, Effect::Refreshing);
                product.effects.add(Effect::Spicy);
            }
            Substance::Iodine => {
                product.effects.replace(&Effect::Calming, Effect::Balding);
                product.effects.replace(&Effect::Toxic, Effect::Sneaky);
                product.effects.replace(&Effect::Foggy, Effect::Paranoia);
                product.effects.replace(&Effect::CalorieDense, Effect::Gingeritis);
                product.effects.replace(&Effect::Euphoric, Effect::SeizureInducing);
                product.effects.replace(&Effect::Refreshing, Effect::ThoughtProvoking);
                product.effects.add(Effect::Jennerising);
            }
            Substance::Paracetamol => {
                product.effects.replace(&Effect::Calming, Effect::Slippery);
                product.effects.replace(&Effect::Toxic, Effect::TropicThunder);
                product.effects.replace(&Effect::Spicy, Effect::BrightEyed);
                product.effects.replace(&Effect::Glowing, Effect::Toxic);
                product.effects.replace(&Effect::Foggy, Effect::Calming);
                product.effects.replace(&Effect::Munchies, Effect::AntiGravity);
                product.effects.replace(&Effect::Electrifying, Effect::Athletic);
                product.effects.add(Effect::Sneaky);
            }
            Substance::Viagra => {
                product.effects.replace(&Effect::Athletic, Effect::Sneaky);
                product.effects.replace(&Effect::Euphoric, Effect::BrightEyed);
                product.effects.replace(&Effect::Laxative, Effect::Calming);
                product.effects.replace(&Effect::Disorienting, Effect::Toxic);
                product.effects.add(Effect::TropicThunder);
            }
            Substance::HorseSemen => {
                product.effects.replace(&Effect::AntiGravity, Effect::Calming);
                product.effects.replace(&Effect::Gingeritis, Effect::Refreshing);
                product.effects.replace(&Effect::ThoughtProvoking, Effect::Electrifying);
                product.effects.add(Effect::LongFaced);
            }
            Substance::MegaBean => {
                product.effects.replace(&Effect::Calming, Effect::Glowing);
                product.effects.replace(&Effect::Sneaky, Effect::Calming);
                product.effects.replace(&Effect::Jennerising, Effect::Paranoia);
                product.effects.replace(&Effect::Athletic, Effect::Laxative);
                product.effects.replace(&Effect::Slippery, Effect::Toxic);
                product.effects.replace(&Effect::ThoughtProvoking, Effect::Cyclopean);
                product.effects.replace(&Effect::Focused, Effect::Disorienting);
                product.effects.replace(&Effect::SeizureInducing, Effect::Focused);
                product.effects.replace(&Effect::Shrinking, Effect::Electrifying);
                product.effects.add(Effect::Foggy);
            }
            Substance::Addy => {
                product.effects.replace(&Effect::Sedating, Effect::Gingeritis);
                product.effects.replace(&Effect::LongFaced, Effect::Electrifying);
                product.effects.replace(&Effect::Glowing, Effect::Refreshing);
                product.effects.replace(&Effect::Foggy, Effect::Energizing);
                product.effects.replace(&Effect::Explosive, Effect::Euphoric);
                product.effects.add(Effect::ThoughtProvoking);
            }
            Substance::Battery => {
                product.effects.replace(&Effect::Munchies, Effect::TropicThunder);
                product.effects.replace(&Effect::Euphoric, Effect::Zombifying);
                product.effects.replace(&Effect::Electrifying, Effect::Euphoric);
                product.effects.replace(&Effect::Laxative, Effect::CalorieDense);
                product.effects.replace(&Effect::Cyclopean, Effect::Glowing);
                product.effects.replace(&Effect::Shrinking, Effect::Munchies);
                product.effects.add(Effect::BrightEyed);
            }
        }
        product
    }

    fn price(&self) -> f64 {
        self.solid.base_price() * (1.0 + self.effects.multiplier())
    }
}

fn main() {
    struct Result {
        seed: Seed,
        substances: [Substance; 3],
        product: Product,
        production_cost: f64,
        selling_price: f64,
        net_profit: f64,
    }

    let mut results = vec![];

    for seed in [Seed::OGKush, Seed::SourDiesel, Seed::GreenCrack, Seed::GranddadyPurple] {
        let product = seed.product();
        for s1 in Substance::iter() {
            let product = product.mix(&s1);
            for s2 in Substance::iter() {
                let product = product.mix(&s2);
                for s3 in Substance::iter() {
                    let product = product.mix(&s3);
                    let production_cost = seed.price() / 12.0 + s1.price() + s2.price() + s3.price();
                    let selling_price = product.price();
                    let net_profit = selling_price - production_cost;
                    results.push(Result {
                        seed: seed,
                        substances: [s1, s2, s3],
                        product,
                        production_cost,
                        selling_price,
                        net_profit,
                    })
                }
            }
        }
    }

    results.sort_by(|a, b| a.net_profit.partial_cmp(&b.net_profit).unwrap().reverse());

    for result in results {
        let effects = itertools::join(result.product.effects.iter().map(|e| format!("{e:?}")), ", ");
        println!(
            "{:?} + {:?} + {:?} + {:?} ([{}]): ${:.01} - ${:.01} = ${:.01}",
            result.seed, result.substances[0], result.substances[1], result.substances[2], effects, result.selling_price, result.production_cost, result.net_profit
        );
    }
}
