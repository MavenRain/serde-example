#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(& point).unwrap_or("{}".to_owned());
    println!("serialized = {}", serialized);
    let deserialized: Point = serde_json::from_str(& serialized).unwrap_or(Point { x: 0, y: 0 });
    println!("deserialized = {:?}", deserialized);
}
