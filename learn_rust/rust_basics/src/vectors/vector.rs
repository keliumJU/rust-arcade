pub fn show_vector_ex() {
    //Declare classic vec that grow or shrink
    let mut nums = vec![15, 3, 2];
    println!("Initial vector: {:?}", nums);

    //Declare fixed vector
    let zeros = vec![0; 5];
    println!("Zeros: {:?}", zeros);

    //Create empty vector
    let mut fruit = Vec::new();

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);


    //Remove an element from vector
    println!("Pop off: {:?}", fruit.pop());
    println!("Current fruits: {:?}", fruit);


    //Access by index
    let three = nums[1];
    println!("Second value of nums vector: {}", three);

    //Update value of vector by index
    nums[0] = nums[0] + 5;
    println!("First value of nums vector: {}, {:?}", nums[0], nums);


    //Add value to array instanciate with vec primitive
    nums.push(7);
    println!("Nums: {:?}", nums);

}
