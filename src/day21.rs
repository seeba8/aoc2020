/*
1. Make list of allergens
2. for each recipe, add all ingredients to each allergen (HashMap?)
3. If recipe has allergen but does not contain ingredient in allergen map entry, remove entry
4. If allergen only has one ingredient, remove that ingredient from other allergens

 */
use std::collections::{HashMap, HashSet};

/**
Returns a map { Allergen => Ingredients }
*/
pub fn get_allergen_map(input: &str) -> Option<HashMap<&str, HashSet<&str>>> {
    let mut allergen_map = HashMap::new();
    for line in input.lines() {
        match line.trim().split_once(" (contains ") {
            None => {
                // no known allergens for this recipe
            }
            Some((ingredients, allergens)) => {
                let allergens = allergens.trim_matches(')');
                let ingredients: HashSet<&str> = ingredients.split_whitespace().collect();
                for allergen in allergens.split(", ") {
                    match allergen_map.get_mut(allergen) {
                        None => { allergen_map.insert(allergen, ingredients.clone()); }
                        Some(set) => { set.extend(ingredients.clone()); }
                    }
                }
            }
        };
    }
    Some(allergen_map)
}

fn remove_impossible_options(input: &str, allergen_map: &mut HashMap<&str, HashSet<&str>>) -> Option<()> {
    while allergen_map.iter().any(|(_, ingredients)| ingredients.len() > 1) {
        for recipe in input.lines() {
            match recipe.trim().split_once(" (contains ") {
                None => {
                    // no known allergens for this recipe
                }
                Some((ingredients, allergens)) => {
                    let allergens = allergens.trim_matches(')');
                    let ingredients: HashSet<&str> = ingredients.split_whitespace().collect();
                    for allergen in allergens.split(", ") {
                        allergen_map.get_mut(allergen)?.retain(|&x| ingredients.contains(x));
                    }
                }
            }
        }
        let mut single_ingredients = HashSet::new();
        for (_, single_ingredient) in allergen_map.iter() {
            if single_ingredient.len() == 1 {
                single_ingredients.insert(*single_ingredient.iter().next()?);
            }
        }
        for ingredient in single_ingredients {
            for (_, ingredients) in allergen_map.iter_mut() {
                if ingredients.len() > 1 {
                    ingredients.remove(ingredient);
                }
            }
        }
    }
    Some(())
}

fn get_ingredients(input: &str) -> HashSet<&str> {
    let mut ingredients = HashSet::new();
    for line in input.lines() {
        match line.trim().split_once(" (contains ") {
            None => {
                ingredients.extend(line.trim().split_whitespace());
            }
            Some((ingredient_list, _)) => {
                ingredients.extend(ingredient_list.trim().split_whitespace());
            }
        };
    }
    ingredients
}

fn get_ingredients_without_allergens(input: &str) -> Option<HashSet<&str>> {
    let mut allergen_map = get_allergen_map(input)?;
    remove_impossible_options(input, &mut allergen_map)?;
    let mut all_ingredients = get_ingredients(input);
    for (_, ingredients) in allergen_map.iter() {
        for &ingredient in ingredients.iter() {
            all_ingredients.remove(ingredient);
        }
    }
    Some(all_ingredients)
}

pub fn count_ingredients_without_allergens(input: &str) -> usize {
    let ingredients = match get_ingredients_without_allergens(input) {
        None => { HashSet::new() }
        Some(ingredients) => { ingredients }
    };
    let mut count = 0;
    for line in input.lines() {
        match line.trim().split_once(" (contains ") {
            None => {
                for recipe_ingredient in line.trim().split_whitespace() {
                    if ingredients.contains(recipe_ingredient) {
                        count += 1;
                    }
                }
            }
            Some((ingredient_list, _)) => {
                for recipe_ingredient in ingredient_list.trim().split_whitespace() {
                    if ingredients.contains(recipe_ingredient) {
                        count += 1;
                    }
                }
            }
        };
    }
    count
}

pub fn get_canonical_dangerous_ingredients_list(input: &str) -> Option<String> {
    let mut allergen_map = get_allergen_map(input)?;
    remove_impossible_options(input, &mut allergen_map);
    let mut keys: Vec<&str> = allergen_map.keys().copied().collect();
    keys.sort_unstable();
    let sorted_ingredients: Vec<&str> = keys
        .iter()
        .map(|&k| *allergen_map.get(k).unwrap().iter().next().unwrap())
        .collect();
    Some(sorted_ingredients.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::iter::FromIterator;

    #[test]
    fn test_get_allergens() -> Result<(), Box<dyn Error>> {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        let allergen_map = get_allergen_map(input).ok_or("no allergen_map")?;
        assert!(allergen_map.get("fish").ok_or("no fish")?.contains("kfcds"));
        assert!(allergen_map.get("fish").ok_or("no fish")?.contains("sbzzf"));
        Ok(())
    }

    #[test]
    fn test_get_ingredients() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        let expected = HashSet::from_iter(vec!["mxmxvkd", "kfcds", "sqjhc", "nhms", "trh", "fvjkl", "sbzzf"]);
        assert_eq!(expected, get_ingredients(input));
    }

    #[test]
    fn test_ingredients_without_allergens() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        let expected = HashSet::from_iter(vec!["kfcds", "nhms", "sbzzf", "trh"]);
        assert_eq!(expected, get_ingredients_without_allergens(input).unwrap());
    }

    #[test]
    fn test_count_unproblematic_ingredients() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        assert_eq!(5, count_ingredients_without_allergens(input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("resources/day21.txt").unwrap();
        println!("{}", count_ingredients_without_allergens(&input));
    }

    #[test]
    fn test_get_canonical_dangerous_ingredients() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        assert_eq!(String::from("mxmxvkd,sqjhc,fvjkl"), get_canonical_dangerous_ingredients_list(input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("resources/day21.txt").unwrap();
        println!("{}", get_canonical_dangerous_ingredients_list(&input).unwrap());
    }
}