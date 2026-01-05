use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
};

type Graph<'a> = HashMap<Cow<'a, String>, Vec<Cow<'a, String>>>;
type InDegree<'a> = HashMap<Cow<'a, String>, i32>;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let (graph, mut indegree) = Self::build_graph(&recipes, &ingredients);
        let recipes = recipes.iter().collect::<HashSet<_>>();
        let supplies = supplies.into_iter().collect::<HashSet<_>>();
        let mut queue = supplies.iter().collect::<VecDeque<_>>();
        let mut result = Vec::new();
        while let Some(supply) = queue.pop_front() {
            if recipes.contains(supply) {
                result.push(supply.clone());
            }
            for adjacent in graph.get(supply).into_iter().flatten() {
                let node_indegree = indegree.get_mut(adjacent).unwrap();
                *node_indegree -= 1;
                if *node_indegree == 0 {
                    queue.push_back(adjacent);
                }
            }
        }
        result
    }

    fn build_graph<'a>(
        recipes: &'a [String],
        ingredients: &'a [Vec<String>],
    ) -> (Graph<'a>, InDegree<'a>) {
        let mut graph = HashMap::new();
        let mut indegree = HashMap::new();
        for (recipe, recipe_ingredient) in recipes.iter().zip(ingredients.iter()) {
            for ingredient in recipe_ingredient {
                graph
                    .entry(Cow::Borrowed(ingredient))
                    .or_insert_with(Vec::new)
                    .push(Cow::Borrowed(recipe));
                *indegree.entry(Cow::Borrowed(recipe)).or_insert(0) += 1;
            }
        }
        // In graph, key is ingredient, value is recipe containing the ingredient
        // Indegree is the number of ingredients in each recipe
        (graph, indegree)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let recipes = ["bread"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let ingredients = [["yeast", "flour"]]
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.to_string()).collect())
        .collect::<Vec<_>>();
    let supplies = ["yeast", "flour", "corn"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    tracing::info!(
        "{:?}",
        Solution::find_all_recipes(recipes, ingredients, supplies)
    )
}
