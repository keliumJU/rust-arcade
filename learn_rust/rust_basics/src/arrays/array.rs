pub fn show_array_ex() {
    // Simple array
    let days = ["monday", "tuesday", "wenesday", "tursday", "friday", "saturday", "sunday"];
    println!("Simple array: {}", days[0]);

    //Define array with length
    let nums = [0; 5];
    println!("Fixed array: {}", nums[0]);
}
