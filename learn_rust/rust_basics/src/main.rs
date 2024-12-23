mod models;
//mod arrays;
//mod vectors;
//mod maps;

//use crate::models::car::{
//    show_car_ex,
//};

//use crate::arrays::array::{
//    show_array_ex,
//};

//use crate::vectors::vector::{
//    show_vector_ex,
//};

//use crate::maps::map::{
//    show_map_ex,
//};

//use crate::models::person::{
//    show_person_ex,
//};

//use crate::models::result::{
//   show_struct_result_ex, 
//};

//use crate::models::file::{
//    show_read_file_ex,
//};

//use crate::models::lifetime::{
//    //show_lifetime_ex,
//    show_lifetime_problem,
//};

//use crate::models::generics::{
//    show_generics_ex,
//};

//use crate::models::point::{
//    show_point_ex,
//};

//use crate::models::asjson::{
//    show_as_json_ex,
//};

//use crate::models::iterator::{
//    show_iterator_ex,
//};

//use crate::models::container::{
//    show_container_ex,
//};

//use crate::models::groups::{
//    show_groups_ex,
//};

//use crate::models::regex::{
//    show_regex_ex,
//};

mod car_factory {
    pub fn build_car() {
        println!("Honk honk!")
    }
    
}

mod text_processing {
    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text); 
    (number_of_letters, number_of_numbers)
}

fn main() {

    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
    //car_factory::build_car();
    //show_regex_ex();
    //show_groups_ex();
    //show_container_ex();
    //show_iterator_ex();
    //show_as_json_ex();
    //show_point_ex();
    //show_generics_ex();
    //show_lifetime_problem();
    //show_lifetime_ex();
    //show_read_file_ex();
    //show_struct_result_ex();
    //show_person_ex();
    //Show a panic message
    //panic!("Farewell!!");

}
