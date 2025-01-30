use core::fmt;
use core::fmt::Display;

use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use axum::{routing::get, Json, Router};

#[derive(Deserialize, Serialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

/// Here we get a post
/// # Returns
/// A post
/// # Errors
/// If the post office is not operational
fn get_post() -> Result<Post, String> {
    Ok(Post {
        userId: 1,
        id: 1,
        title: "foo".into(),
        body: "bar".into(),
    })
}

async fn post_handler() -> Json<Post> {
    let post = get_post().unwrap();
    Json(post)
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

    // let mut file = File::open("post.json").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();

    // let post: Post = serde_json::from_str(&contents).unwrap();

    // println!("Post: {:?}", post);

    // build our application with a single route
    let app = Router::new().route("/", get(post_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_post() {
        let post = get_post().unwrap();
        assert_eq!(post.userId, 1);
    }
}
