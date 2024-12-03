use std::collections::HashMap;

pub fn show_map_ex() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate"));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet Recipents"));

    //Get key value
    let book: &str = "Ancient Roman History";
    println!("Review for: {}: {:?}", book, reviews.get(book));

    //Remove key-value pair
    let obsolete: &str = "Cooking with Rhubarb";
    reviews.remove(obsolete);

    println!("Obsolete: {}, {:?}", obsolete, reviews.get(obsolete));
}
