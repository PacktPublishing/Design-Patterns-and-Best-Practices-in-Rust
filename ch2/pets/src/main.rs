trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
}

// Misusing generics to emulate a base class
struct Pet<T: Animal> {
    name: String,
    animal: T,
}

impl<T: Animal> Pet<T> {
    fn new(name: String, animal: T) -> Self {
        Pet { name, animal }
    }

    fn speak(&self) -> String {
        format!("{} says: {}", self.name, self.animal.make_sound())
    }
}

fn main() {
    let dog_pet = Pet::new("Buddy".to_string(), Dog);
    let cat_pet = Pet::new("Whiskers".to_string(), Cat);

    println!("{}", dog_pet.speak());
    println!("{}", cat_pet.speak());

    // Attempt to create a vector of different pets
    let pets: Vec<Pet<_>> = vec![dog_pet, cat_pet];
}
