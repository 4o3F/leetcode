use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

use itertools::izip;

#[derive(Default, Clone)]
struct Food {
    name: String,
    cuisine: String,
    rating: i32,
}

impl Eq for Food {}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.cuisine == other.cuisine && self.rating == other.rating
    }
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rating == other.rating {
            Some(other.name.cmp(&self.name))
        } else {
            self.rating.partial_cmp(&other.rating)
        }
    }
}

struct FoodRatings {
    cuisine: HashMap<String, BTreeSet<Rc<Food>>>,
    food: HashMap<String, Rc<Food>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_map = HashMap::<String, BTreeSet<Rc<Food>>>::new();
        let mut food_map = HashMap::<String, Rc<Food>>::new();
        for (food, cuisine, rating) in izip!(foods, cuisines, ratings) {
            let food = Rc::new(Food {
                name: food,
                cuisine: cuisine.clone(),
                rating,
            });

            cuisine_map
                .entry(cuisine)
                .or_default()
                .insert(Rc::clone(&food));

            food_map.insert(food.name.clone(), food);
        }
        FoodRatings {
            cuisine: cuisine_map,
            food: food_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(old_food_rc) = self.food.get(&food) {
            let old_food = Rc::clone(old_food_rc);
            let cuisine = old_food.cuisine.clone();

            if let Some(cuisine_set) = self.cuisine.get_mut(&cuisine) {
                cuisine_set.remove(&old_food);
            }

            let new_food = Rc::new(Food {
                name: old_food.name.clone(),
                cuisine,
                rating: new_rating,
            });

            self.food.insert(food, Rc::clone(&new_food));
            if let Some(cuisine_set) = self.cuisine.get_mut(&new_food.cuisine) {
                cuisine_set.insert(Rc::clone(&new_food));
            }
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        if let Some(cuisine_set) = self.cuisine.get(&cuisine) {
            if let Some(highest) = cuisine_set.iter().last() {
                return highest.name.clone();
            }
        }
        unreachable!()
    }
}

pub fn run() {
    let mut obj = FoodRatings::new(
        vec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
        vec![
            "korean", "japanese", "japanese", "greek", "japanese", "korean",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect(),
        vec![9, 12, 8, 15, 14, 7],
    );
    tracing::info!("{}", obj.highest_rated("korean".to_string()));
    tracing::info!("{}", obj.highest_rated("japanese".to_string()));
    obj.change_rating("sushi".to_string(), 16);
    tracing::info!("{}", obj.highest_rated("japanese".to_string()));
    obj.change_rating("ramen".to_string(), 16);
    tracing::info!("{}", obj.highest_rated("japanese".to_string()));
}

// [
//     [
//         ["kimchi","miso","sushi","moussaka","ramen","bulgogi"],
//         ["korean","japanese","japanese","greek","japanese","korean"],
//         [9,12,8,15,14,7]
//     ],
//     ["korean"],
//     ["japanese"],
//     ["sushi",16],
//     ["japanese"],
//     ["ramen",16],
//     ["japanese"]
// ]

// ["FoodRatings","highestRated","highestRated","changeRating","highestRated","changeRating","highestRated"]
