# CodeWars-Pete-the-baker--5-kyu---Passed
Pete likes to bake some cakes. He has some recipes and ingredients. Unfortunately he is not good in maths. Can you help him to find out, how many cakes he could bake considering his recipes?

Write a function cakes(), which takes the recipe (HashMap<&str, u32>) and the available ingredients (also a HashMap<&str, u32>) and returns the maximum number of cakes Pete can bake (u32). For simplicity there are no units for the amounts (e.g. 1 lb of flour or 200 g of sugar are simply 1 or 200). Ingredients that are not present in the objects, can be considered as 0.

Examples:

// must return 2
cakes(HashMap::from([("flour", 500), ("sugar", 200), ("eggs", 1)]), HashMap::from([("flour", 1200), ("sugar", 1200), ("eggs", 5), ("milk", 200)]))
// must return 0
cakes(HashMap::from([("apples", 3), ("flour", 300), ("sugar", 150), ("milk", 100), ("oil", 100)]), HashMap::from([("sugar", 500),("flour", 2000), ("milk", 2000)]))

TEST CASES:
#[cfg(test)]
mod tests {
    use super::cakes;
    use std::collections::HashMap;
    use rand::prelude::*;
    
    macro_rules! map {
        () => { HashMap::new() };
        ($($ingredient:ident : $amount:expr),+) => {{
            let mut map = HashMap::new();
            $( map.insert(stringify!($ingredient), $amount); )*
            map
        }};
    }
    
    fn test(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>, expected: u32) {
        let actual = cakes(recipe, available);
        assert!(actual == expected, "Expected to bake {expected} cakes, but got {actual} cakes instead.\nAvailable ingredients:\n  {available:?}\nRecipe:\n  {recipe:?}\n\n");
    }

    #[test]
    fn test_basic() {
        let recipe = map!(flour: 500, sugar: 200, eggs: 1);
        let available = map!(flour: 1200, sugar: 1200, eggs: 5, milk: 200);
        test(&recipe, &available, 2);
        
        let recipe = map!(cream: 200, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 1700, flour: 20000, milk: 20000, oil: 30000, cream: 5000);
        test(&recipe, &available, 11);
    }
    
    #[test]
    fn test_missing_ingredient() {
        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000);
        test(&recipe, &available, 0);
    }
    
    #[test]
    fn test_not_enough_ingredients() {
        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000, apples: 15, oil: 20);
        test(&recipe, &available, 0);
    }
    
    #[test]
    fn test_no_ingredients_available() {
        let recipe = map!(eggs: 4, flour: 400);
        let available = map!();
        test(&recipe, &available, 0);
    }
    
    #[test]
    fn test_random() {
        let mut rng = thread_rng();
        for _ in 0..200 {
            if rng.gen_bool(0.7) {
                test_positive();
            } else {
                test_negative();
            }
        }
    }
    
    const MAX_AMOUNT: u32 = 100;
    
    /// Creates a tuple of (available ingredients, recipe ingredients, unused ingredients)
    fn random_recipe() -> (HashMap<&'static str, u32>, HashMap<&'static str, u32>, Vec<&'static str>) {
        let mut rng = thread_rng();
        let mut ingredients = ["flour", "eggs", "oil", "milk", "apples", "sugar", "cream", "pears", "butter", "nuts", "chocolate", "cocoa", "crumbles"];
        ingredients.shuffle(&mut rng);
        // Partition at some index into available and the rest
        let partition_point = rng.gen_range(1..13);
        let available: HashMap<&str, u32> = HashMap::from_iter(
            ingredients[..partition_point]
                .iter()
                .map(|&ingr| (ingr, rng.gen_range(1..=MAX_AMOUNT))),
        );
        // Choose a subset of available ingredients for the recipe
        let r_count = rng.gen_range(1..=available.len());
        let used: Vec<&str> = ingredients[..partition_point]
            .choose_multiple(&mut rng, r_count)
            .copied().collect();
        // Choose a random number of cakes based on minimum available ingredient;
        // this skews the expected result away from a likely 1. This is not the exact result.
        let (min_amount, min_ingr) = used.iter()
            .map(|&ingr| (available[ingr], ingr))
            .min().unwrap();
        let cake_count = rng.gen_range(1..=10.min(min_amount));
        let recipe: HashMap<&str, u32> = HashMap::from_iter(used.into_iter().map(|ingr| {
            (ingr, if ingr == min_ingr {
                // exact for the minimum ingredient
                min_amount / cake_count
            } else {
                // random, but limited, for all others
                rng.gen_range(1..=available[ingr] / cake_count)
            })
        }));
        (
            available,
            recipe,
            Vec::from(&ingredients[partition_point..]),
        )
    }
    
    fn test_positive() {
        let (available, recipe, _) = random_recipe();
        let expected = recipe.iter()
            .map(|(key, value)| available.get(key).unwrap_or(&0) / value)
            .min()
            .unwrap_or(0);
        test(&recipe, &available, expected);
    }
    
    fn test_negative() {
        let mut rng = thread_rng();
        let (available, mut recipe, rest) = random_recipe();
        if rng.gen_bool(0.5) {
            // add unavailable ingredients from the rest
            let r_count = rng.gen_range(1..=rest.len());
            recipe.extend(rest.choose_multiple(&mut rng, r_count).map(|&ingr| (ingr, rng.gen_range(1..=MAX_AMOUNT))));
        } else {
            // increase random required amounts beyond available
            for _ in 0..rng.gen_range(1..=recipe.len()) {
                let (&ingr, amount) = recipe.iter_mut().choose(&mut rng).unwrap();
                *amount += available[ingr] + rng.gen_range(1..50);
            }
        }
        test(&recipe, &available, 0);
    }
 
}
