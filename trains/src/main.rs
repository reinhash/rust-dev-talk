use core::fmt;
use core::fmt::Display;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

struct Train {
    name: String,
    speed: f64,
    type_: TrainType,
}

impl Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Es fährt ein Zug namens {} ein und er hat folgenden Typ {} und fährt mit einer Geschwindigkeit von {}", self.name, self.type_, self.speed)
    }
}

enum TrainType {
    ICE,
    IC,
    Regional(String),
    S,
}

impl Display for TrainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = match self {
            TrainType::ICE => "ICE".into(),
            TrainType::IC => "IC".into(),
            TrainType::Regional(name) => format!("Regional {}", name),
            TrainType::S => "S".into(),
        };
        write!(f, "{}", type_name)
    }
}

fn main() {
    // let train = Train {
    //     name: String::from("Thomas"),
    //     speed: 99.0,
    //     type_: TrainType::Regional(String::from("S-7")),
    // };
    // print!("{}", train);
    let mut file = File::open("post.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let post: Post = serde_json::from_str(&contents).unwrap();

    println!("Post: {:?}", post);
}
