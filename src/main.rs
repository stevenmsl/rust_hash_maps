use std::collections::HashMap;

fn main() {
    create_maps();
    create_maps_from_vecs();
    ownership();
    access_value();
    overwrite();
    insert();
    update_based_on_old_value();
}

fn create_maps() {
    /*
      - Rust will infer the type from the usage
        of "insert" later in the code
    */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores are {:?}", scores);
}

fn create_maps_from_vecs() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    /*
      - we need to type annotate the scores as you can collect
        into many different data structures; Rust needs help.
      - we don't need to specify the type of key or value as Rust
        can infer the types
    */

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores are {:?}", scores);
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // owned values (like String) will be moved
    // into the map
    map.insert(field_name, field_value);
    /*
      - field_name and field_value are invalid
        at this point.
      - the following code won't compile
    */
    //println!("name is {0}", field_name);
}

fn access_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn overwrite() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // replace 10 with 25
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}

fn insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);

    //Blue will remain as 10
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
