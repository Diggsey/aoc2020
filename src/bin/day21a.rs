use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day21.txt");

struct Item {
    ingredients: HashSet<&'static str>,
    allergens: HashSet<&'static str>,
}

fn extract_one(ingredients: &HashSet<&'static str>) -> Option<&'static str> {
    if ingredients.len() != 1 {
        None
    } else {
        Some(*ingredients.iter().next().unwrap())
    }
}

fn record_allergen(
    allergen: &'static str,
    ingredient: &'static str,
    constraints: &mut HashMap<&'static str, HashSet<&'static str>>,
    mapping: &mut HashMap<&'static str, &'static str>,
) {
    mapping.insert(ingredient, allergen);

    let mut recurse = Vec::new();
    for (&allergen2, ingredients2) in constraints.iter_mut() {
        if allergen2 != allergen && ingredients2.remove(&ingredient) {
            if let Some(ingredient2) = extract_one(ingredients2) {
                recurse.push((allergen2, ingredient2));
            }
        }
    }
    for (allergen2, ingredient2) in recurse {
        record_allergen(allergen2, ingredient2, constraints, mapping);
    }
}

fn main() {
    let list: Vec<_> = INPUT
        .lines()
        .map(|l| {
            let mut part_it = l.splitn(2, " (contains ");
            Item {
                ingredients: part_it.next().unwrap().split_whitespace().collect(),
                allergens: part_it
                    .next()
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(", ")
                    .collect(),
            }
        })
        .collect();

    let mut mapping = HashMap::new();
    let mut constraints = HashMap::<_, HashSet<_>>::new();
    for item in &list {
        for allergen in item.allergens.iter().copied() {
            if let Some(ingredient) = extract_one(match constraints.entry(allergen) {
                Entry::Occupied(entry) => {
                    let ingredients = entry.into_mut();
                    ingredients.retain(|ingredient| item.ingredients.contains(ingredient));
                    ingredients
                }
                Entry::Vacant(entry) => entry.insert(item.ingredients.clone()),
            }) {
                record_allergen(allergen, ingredient, &mut constraints, &mut mapping);
            }
        }
    }

    let count = list
        .iter()
        .flat_map(|item| &item.ingredients)
        .copied()
        .filter(|ingredient| !mapping.contains_key(ingredient))
        .count();
    println!("{}", count);
}
