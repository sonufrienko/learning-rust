use p1_hello::my_add;
use rand::Rng;
use std::collections::HashMap;
use std::thread;

const REG_COUNT: u32 = 10;

fn get_random() -> i32 {
    let x: i32 = rand::thread_rng().gen_range(0, 100);
    return x;
}

struct House {
    pool: bool,
    furnished: bool,
    address: String,
    price: f64,
}

enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
}

impl House {
    fn new(price: f64) -> Self {
        Self {
            pool: false,
            furnished: false,
            address: String::from(""),
            price: price,
        }
    }
}

trait Rentals {
    fn rent(&self);
}

impl Rentals for House {
    fn rent(&self) {
        println!("You billed for ${}", self.price)
    }
}

fn main() {
    /*
    Variables
     */
    let mut n: u32 = 2;
    let z: f32 = 5.9912;
    let is_me = true;
    n = REG_COUNT;
    let c = my_add(7, 100);
    println!("c = {}", c);
    println!("n = {}", n);

    let (a, b) = (10, 20);
    let years = (1985, 1989, 2014);
    let (y1, y2, y3) = years;
    println!("y3 = {}", y3);

    let my_name = "Sergey🇬🇧-💂";
    println!("{}", my_name);

    let x: i32 = rand::thread_rng().gen_range(0, 100);
    println!("random = {}", x);

    /*
    Conditions
     */

    if x > 50 {
        println!("MORE")
    } else {
        println!("LESS")
    }

    let title = if x > 50 { "More" } else { "Less" };
    println!("X is {} then 50", title);

    /*
    Loops
     */
    loop {
        let x: i32 = rand::thread_rng().gen_range(0, 100);
        if x == 13 {
            println!("FOUND");
            break;
        }
    }

    while get_random() != 13 {
        println!("Not 13")
    }

    let countries = ["US", "PT", "ES", "FR"];
    for country in countries.iter() {
        println!("country = {}", country)
    }

    let points = [(1, 2), (3, 4)];
    for (x, y) in points.iter() {
        println!("x={}, y={}", x, y)
    }

    for i in 0..=50 {
        print!("{} ", i)
    }

    /*
    Struct
     */
    let h1 = House::new(1050.99);
    println!("\n\nh1 price = {}", h1.price);
    h1.rent();
    rent_anything(h1);

    /*
    Collections
     */
    let mut lucky_numbers: Vec<i32> = Vec::new();
    lucky_numbers.push(get_random());
    lucky_numbers.push(get_random());
    lucky_numbers.push(get_random());
    for number in lucky_numbers.iter() {
        println!("Number {} win!", number);
    }

    let error_codes = vec![400, 401, 403, 404, 501, 503];
    let access_problem = error_codes.contains(&401) || error_codes.contains(&403);
    println!("Is this access problem: {}", access_problem);

    let mut cache: HashMap<i32, String> = HashMap::new();
    cache.insert(100, "Cache value for key 100".to_string());
    if let Some(cache_value) = cache.get(&1030) {
        println!("Cache value = {}", cache_value);
    } else {
        println!("Cache not exists");
    }

    /*
    Enum
     */
    fetch(HttpMethod::POST, "https://a.com".to_string());
    fetch(HttpMethod::GET, "https://a.com".to_string());

    let mut x = None;
    x = Some(5);
    println!("is_some = {}", x.is_some());
    println!("is_none = {}", x.is_none());

    let my_var = Some(404);
    let new_var = match my_var {
        Some(x) => true,
        None => false,
    };
    println!("new_var = {}", new_var)
}

fn rent_anything<T: Rentals>(item: T) {
    item.rent()
}

fn fetch(method: HttpMethod, url: String) {
    if matches!(method, HttpMethod::POST) {
        println!("POST {}", url);
    } else {
        println!("Method not supported");
    }
}
