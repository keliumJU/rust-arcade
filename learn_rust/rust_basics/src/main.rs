//Structs
//Clasic struct
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

//Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;

// Enum and enum variants with defined structs

// Define a tuple struct, the directive debug is for check the data in the output :)
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick),
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // return early
        return 0;
    }
    num / 5
}

fn main() {
    let mut a_number;
    let a_word = "Ten";

    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    a_number = 15;

    println!("The number is now {}.", a_number);

    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    let number_64 = 4.0;
    let number_32: f32 = 5.0;

    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    let uppercase_s = 'S';
    let lowercase_ = 'f';
    let smiley_face = '😃';

    // strings &str -> it's a point to immutable string data
    let character_1: char = 'S';
    let character_2: char = 'f';

    let smiley_face = '😃';
    let string_1 = "miley";
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}",
        smiley_face, character_1, string_1, character_2, string_2
    );

    //Data collections
    //the tuple have fix size and are immutable :)
    let tuple_e = ('E', 5i32, true);
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    //Instance clasic structs
    let user_1 = Student {
        name: String::from("Constance Sharma"),
        remote: true,
        level: 2,
    };
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false,
    };

    //Instance tuple structs
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}, Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );

    //Instantiate an enum
    let we_load = WebEvent::WELoad(true);
    //Struct variant
    let click = MouseClick { x: 100, y: 250 };
    let we_click = WebEvent::WEClick(click);
    //Tuple variant
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKeys(keys);

    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );

    let formal = "Formal: Goodbye";
    let casual = "Casual: See you later";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("{} divide by 5 = {}", num, divide_by_5(num));
}
