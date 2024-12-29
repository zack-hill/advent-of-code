use crate::solver::AoCSolver;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

pub struct Solver {
    foods: Vec<Food>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            foods: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let allergen_ingredients = find_allergen_ingredients(&self.foods);
        let allergen_ingredients: HashSet<String> = allergen_ingredients
            .into_iter()
            .map(|(_, ingredient)| ingredient)
            .collect();
        let safe_count = self
            .foods
            .iter()
            .flat_map(|f| &f.ingredients)
            .filter(|i| !allergen_ingredients.contains(*i))
            .count();
        return safe_count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut allergen_ingredients = find_allergen_ingredients(&self.foods);
        allergen_ingredients.sort_by_key(|(allergen, _)| allergen.to_owned());
        let canonical_dangerous_ingredients: Vec<String> = allergen_ingredients
            .iter()
            .map(|(_, ingredient)| ingredient.to_owned())
            .collect();
        return canonical_dangerous_ingredients.join(",");
    }
}

fn find_allergen_ingredients(foods: &Vec<Food>) -> Vec<(String, String)> {
    // TODO: This function feels like a mess. Lots of string copying and data transformations. This should be revisited with a clear head and new tricks.

    let all_allergens: HashSet<String> = foods.iter().flat_map(|f| &f.allergens).cloned().collect();
    let all_ingredients: HashSet<String> =
        foods.iter().flat_map(|f| &f.ingredients).cloned().collect();

    // A map from allergen to a set of possible ingredients, initialized to all possible ingredients
    let mut possible_ingredients_map: HashMap<&str, HashSet<String>> = all_allergens
        .iter()
        .map(|a| (a.as_str(), all_ingredients.clone()))
        .collect();

    // Iterate through the food and update the possible ingredients map based on ingredients and allergens
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            let possible_ingredients = possible_ingredients_map.get(allergen.as_str()).unwrap();
            possible_ingredients_map.insert(
                allergen,
                possible_ingredients
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect(),
            );
        }
    }

    // Loop through the allergens checking for any with a single possible ingredient. Identified ingredients
    // are removed from other allergens. This process eventually reduces the number of possible ingredients
    // for a each allergen down to one.
    let mut unsolved_allergens: VecDeque<&str> = all_allergens.iter().map(|a| a.as_str()).collect();
    while let Some(allergen) = unsolved_allergens.pop_front() {
        let possible_ingredients = &possible_ingredients_map[allergen];
        if possible_ingredients.len() == 1 {
            let ingredient = possible_ingredients.iter().next().unwrap().clone();
            for other in unsolved_allergens.iter() {
                possible_ingredients_map
                    .get_mut(other)
                    .unwrap()
                    .remove(ingredient.as_str());
            }
        } else {
            unsolved_allergens.push_back(allergen);
        }
    }

    // Return pairs of allergen and their matching ingredient.
    return possible_ingredients_map
        .iter()
        .map(|(allergen, ingredients)| {
            (
                allergen.to_string(),
                ingredients.iter().next().unwrap().to_owned(),
            )
        })
        .collect();
}

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_input() -> Vec<Food> {
    let file = File::open("src/2020/day_21.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| parse_food(&line.unwrap()).unwrap().1)
        .collect()
}

fn parse_food(i: &str) -> IResult<&str, Food> {
    let (i, ingredients) = parse_words(i)?;
    let (i, allergens) = parse_allergens(i)?;
    Ok((
        i,
        Food {
            ingredients: HashSet::from_iter(ingredients.iter().map(|x| x.to_string())),
            allergens: HashSet::from_iter(allergens.iter().map(|x| x.to_string())),
        },
    ))
}

fn parse_allergens(i: &str) -> IResult<&str, Vec<&str>> {
    delimited(tag("(contains"), parse_words, tag(")"))(i)
}

fn parse_words(i: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        multispace0,
        separated_list1(alt((tag(" "), tag(", "))), alpha1),
        multispace0,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_words_works() {
        let (text, words) = parse_words("aksmd jfias, asdunas").unwrap();

        assert_eq!("", text);
        assert_eq!(vec!["aksmd", "jfias", "asdunas"], words);
    }

    #[test]
    fn parse_allergens_works() {
        let (text, words) = parse_allergens("(contains soy, dairy)").unwrap();

        assert_eq!("", text);
        assert_eq!(vec!["soy", "dairy"], words);
    }

    #[test]
    fn parse_food_works() {
        let expected_ingredients: HashSet<String> = vec!["nmjbg", "qbm", "vhgtl", "mpbb"]
            .into_iter()
            .map(|a| a.to_string())
            .collect();
        let expected_allergens: HashSet<String> = vec!["shellfish", "peanuts"]
            .into_iter()
            .map(|a| a.to_string())
            .collect();

        let (text, food) =
            parse_food("nmjbg qbm vhgtl mpbb (contains shellfish, peanuts)").unwrap();

        assert_eq!("", text);
        assert_eq!(expected_ingredients, food.ingredients);
        assert_eq!(expected_allergens, food.allergens);
    }
}
