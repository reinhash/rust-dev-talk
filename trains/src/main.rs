use core::fmt;
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use axum::{routing::get, Json, Router};

#[derive(Deserialize, Serialize, Debug)]
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
            TrainType::S => "S-Bahn".into(),
        };
        write!(f, "{}", type_name)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Train {
    name: String,
    speed: f64,
    type_: TrainType,
}

impl Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Es fährt ein Zug namens {} ein und er hat folgenden Typ {}. Er kommt mit {} km/h eingefahren",
        self.name, self.type_, self.speed)
    }
}

/// Here we get a train
/// # Returns
/// A Train
/// # Errors
/// If the train company is on strike
fn get_train() -> Result<Train, String> {
    let mut file = File::open("train.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let train: Train = serde_json::from_str(&contents).unwrap();
    Ok(train)
}

async fn train_handler() -> Json<Train> {
    let train = get_train().unwrap();
    Json(train)
}

/// This is the main function
#[tokio::main]
async fn main() {
    // let train = Train {
    //     name: String::from("Thomas"),
    //     speed: 99.0,
    //     type_: TrainType::Regional(String::from("S-7")),
    // };

    // print!("{}", train);

    // let mut file = File::open("train.json").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();

    // let train: Train = serde_json::from_str(&contents).unwrap();

    // println!("Train: {:?}", train);
    match get_train() {
        Ok(train) => println!("Train found: {:?}", train),
        Err(message) => println!("Error: {}", message),
    };

    // build our application with a single route
    let app = Router::new().route("/", get(train_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_train() {
        let train = get_train().unwrap();
        assert_eq!(train.name, "ICE 1 - Thomas die Lokomotive");
    }
}
