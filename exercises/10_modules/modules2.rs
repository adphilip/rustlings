// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.


    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

    use self::delicious_snacks::fruits::PEAR as PEAR_FRUIT;
    use self::delicious_snacks::veggies::CUCUMBER as CUCUMBER_VEGGIE;
    
fn main() {

    println!(
        "favorite snacks: {} and {}",
        PEAR_FRUIT,
        CUCUMBER_VEGGIE,
    );
}
