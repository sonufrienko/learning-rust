use std::collections::HashMap;

fn main() {
    // Array
    let lucky_numbers: [i32; 3] = [5, 8, 12];

    if lucky_numbers.is_empty() {
        println!("Array empty")
    } else {
        println!("Array not emepty")
    }
    println!("Array have {} items", lucky_numbers.len());

    let my_number = 121;
    let has_my = lucky_numbers.contains(&my_number);
    println!("Array has my number: {}", has_my);

    let my_index = lucky_numbers.iter().position(|&v| v == my_number);
    println!("Index of my number: {}", my_index.unwrap_or(0));

    let sum: i32 = lucky_numbers.iter().sum();
    println!("Array sum: {}", sum);

    let slice = &lucky_numbers[0..2];
    println!("Slice: {:?}", slice);

    println!("---");

    // Vec
    let new_friends = vec!["Max", "Lora"];
    let mut friends = vec!["Sergii", "Nick", "Alex", "Rex"];
    friends.push("Antonio");
    friends.sort();
    println!("Sorted friends:\n{:?}", friends);

    friends.extend(new_friends);
    friends.sort();
    println!("All friends:\n{:?}\nCount: {}", friends, friends.len());

    println!("You have Rex in friends: {}", friends.contains(&"Rex"));

    friends.push("Rex");
    println!("You hace duplications: {:?}", friends);
    friends.sort();
    friends.dedup();
    println!("Duplications removed: {:?}", friends);

    let messages = friends
        .iter()
        .filter(|friend| friend != &&"Rex")
        .map(|friend| format!("Hello {}!", friend));

    for msg in messages {
        println!("{}", msg)
    }

    // HashMap
    let mut cache: HashMap<String, String> = HashMap::new();
    cache.insert(
        String::from("user:1"),
        String::from("{ `\"name\": \"Sergii\" }"),
    );

    let cache_key1 = String::from("user:1");
    let cache_key2 = String::from("user:2");
    if cache.contains_key(&cache_key1) {
        println!("{} key exists", cache_key1);
    }

    let cache_value1 = cache.get(&cache_key1);
    let cache_value2 = cache.get(&cache_key2);

    if cache_value1.is_some() {
        println!("cache_value1 = {}", cache_value1.unwrap())
    }

    println!(
        "cache_value2 or default = {}",
        cache_value2
            .or(Some(&String::from("default-value")))
            .unwrap()
    );

    cache
        .entry(cache_key2.clone())
        .or_insert(String::from("{ \"name\": \"New Alex\" }"));

    println!("value2 = {}", cache.get(&cache_key2).unwrap());

    *cache.get_mut(&cache_key2).unwrap() = String::from("{ \"name\": \"NewName1\" }");

    println!("value2 = {}", cache.get(&cache_key2).unwrap());

    if let Some(val) = cache.get_mut(&cache_key2) {
        *val = String::from("{ \"name\": \"NewName2\" }");
    }
    println!("value2 = {}", cache.get(&cache_key2).unwrap());
}
