use std::vec;
use std::env;
use std::collections::HashMap;
use library::*;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

struct Person {
    person_name: String,
    person_weight: u64,
    person_address: String,
    ethnicity: Color,
}

impl Person {
    fn weight_person(&self,other_person: &Person)->bool{
        self.person_weight > other_person.person_weight

    }
}          

enum Color {
    White,
    AfricanAmerican,
}

fn main() {
    let person1=Person{
        person_name :String::from("Nikos Nikou"),
        person_weight : 63,
        person_address : String::from("Wolf Street"),
        ethnicity:Color::AfricanAmerican,
    };

    let person2=Person{
        person_name :String::from("Giannis Giannou"),
        person_weight : 65,
        person_address : String::from("Lion Street"),
        ethnicity:Color::White,
    };

    println!("\nThe people are: ");
    println!("{}\n{} kilos\n{}\n",person2.person_name,person2.person_weight,person2.person_address);
    println!("{}\n{} kilos\n{}",person1.person_name,person1.person_weight,person1.person_address);

    println!("\nIs {} fatter than {}? {}",person1.person_weight,person2.person_weight,person1.weight_person(&person2));
        for person in [&person1,&person2]{
        print!("\n{} is: ",person.person_name);
        match person.ethnicity {
            Color::AfricanAmerican=> println!("From a black family"),
            Color::White=>println!("From a white family"),
        }
    }
    let some_u8_value: Option<i32> = Some(3);
    if let Some(3) = some_u8_value {
        println!("Match case without match.\n");
        }

let mut hashmap: HashMap<String, i32> = HashMap::new();
hashmap.insert(String::from("Age of Nikos"), 100);
hashmap.insert(String::from("Age of Giannis"), 25);

for (key, value) in &hashmap {
    println!("{}: {}", key, value);
}

let number_list: Vec<i32>=vec![5,6,7,8,2,99];
let res: i32 = largest(&number_list);
println!("The largest number in {:?}  is  {}",number_list,res);

let char_list: Vec<char>=vec!['a','b','c','d','e'];
let res: char = largest(&char_list);
println!("The largest char in {:?}  is  {}",char_list,res);

let string1 = String::from("abcd");
let string2 = "xyz";
let result = longest(string1.as_str(), string2);

thread::sleep(Duration::from_secs(2));
println!("The longest string is {}", result);

let args: Vec<String> = env::args().collect();

let filename = &args[2];
let mut f = File::open(filename).expect("file not found");
let mut contents = String::new();
f.read_to_string(&mut contents)
.expect("something went wrong reading the file");
println!("\nWith text:\n{}", contents);

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

let c = CustomSmartPointer { data: String::from("my stuff") };
let d = CustomSmartPointer { data: String::from("other stuff") };


}

