/// Trait for all fruits
trait fruit {

    fn contains_seeds(&self) -> bool;
}

struct apple {
    color: String,
    seedless: bool,
}

impl fruit for apple {
    fn contains_seeds(&self) -> bool {
        self.seedless
    }
}


struct grape {
    seedless: bool,
    good_for_wine: bool,
    avg_diameter: f64
}

impl fruit for grape {
    fn contains_seeds(&self) -> bool {
        self.seedless
    }
}


fn main() {
    println!("Create structs for a given trait and store them in a dynamic Vec");
    let mut fruits = Vec::<Box<dyn fruit>>::new();

    let granny_smith = apple {
        color: "green".to_owned(),
        seedless: false,
    };

    let white_grape = grape {
        seedless: true,
        good_for_wine: false,
        avg_diameter: 1.7
    };

    fruits.push(Box::new(granny_smith));
    fruits.push(Box::new(white_grape));

    println!("All fruits pushed to Vec!");

    println!("Seedless: {:?}", fruits[0].contains_seeds())
}

