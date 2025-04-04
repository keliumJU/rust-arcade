pub struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

pub fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    match &person.middle {
        Some(middle) => {
            full_name.push_str(&middle);
            full_name.push_str(" ");
        },
        None => ()
    } 

    full_name.push_str(&person.last);
    full_name
}

pub fn show_person_ex() {
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };

    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };

    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };

    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
