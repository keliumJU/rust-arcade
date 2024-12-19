#[derive(Debug)]
struct Hightlight<'document>(&'document str);

fn erase(_: String) { }


pub fn show_lifetime_ex() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Hightlight(&text[4..19]);
    let dog = Hightlight(&text[35..43]);

    //erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}

fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

pub fn show_lifetime_problem() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
        );
}
