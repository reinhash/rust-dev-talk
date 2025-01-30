# Presentation guide für Reinhard's Rust Rant


## The borrow checker
1. create train struct
```rust
struct Train {
    name: String,
    speed: f64,
}
```
2. initialize train struct from main
```rust
let train = Train {
  name: String::from("Thomas"),
  speed: 99.0,
};
```
3. print the train name
```rust
println!("{}", train.name);
```
4. print it again
```rust
println!("{}", train.name);
```
**Does it compile?**

Observe that all is good!

3. create a new printer function
```rust
fn printer(train: Train) {
    println!("{}", train.name);
}
```
4. print with the new function
```rust
printer(train);
```

**Does it compile?**

5. print again with the new function
```rust
printer(train);
```

**Does it compile?**

Observe the error message

Explanation:
You just moved the value into the function on the first call. Therefore, it is not available anymore on the second call.

6. Now lets borrow it to solve the problem
```rust
printer(&train);
printer(&train);
```
Observe that the function signature needs to change as well.

7. Update the function signature
```rust
fn printer(train: &Train) {
    println!("{}", train.name);
}
```
You just learned that a function can borrow!

## Enums

8. Create an enum of train type
```rust
enum TrainType {
    ICE,
    IC,
    Regional,
    S,
}
```

9. Add the train type to the Train struct
```rust
struct Train {
    name: String,
    speed: f64,
    type_: TrainType,
}
```

10. Run ```cargo check``` and notice that we receive an error

11. Add the train type to the instance
```rust
let train = Train {
    name: String::from("Thomas"),
    speed: 99.0,
    type_: TrainType::Regional,
};
```

## Nested Enums

12. Now lets add some magic to the enum and let the Regional Train Type wrap the String type
```rust
enum TrainType {
    ICE,
    IC,
    Regional(String),
    S,
}
```

13. Let's also fix the initialization
```rust
let train = Train {
    name: String::from("Thomas"),
    speed: 99.0,
    type_: TrainType::Regional(String::from("S-7")),
};
```

14. Let's see how we can print the train type in the printer function
```rust
fn printer(train: &Train) {
    let name = &train.name;
    let speed = &train.speed;
    let type_name = match &train.type_ {
        TrainType::ICE => "ICE".into(),
        TrainType::IC => "IC".into(),
        TrainType::Regional(name) => format!("Regional {}", name),
        TrainType::S => "S-Bahn".into(),
    };
    println!(
        "Es fährt ein Zug namens {} ein und er hat folgenden Typ {}. Er kommt mit {} km/h eingefahren",
        name, type_name, speed
    );
}
```
As you can see the power of pattern matching is quite amazing. But the locality of logic in this code is very bad. Let's refactor it!

## Trait implementations

15. Create an Display implementation for the enum TrainType

- first, import the display trait ```use core::fmt::Display;```
- second, import the formatter needed for the implementation: ```use core::fmt;```
- third: implement it on the enum:
```rust
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
```

16. Use the Display implementation to print the type in the printer
```rust
fn printer(train: &Train) {
    let name = &train.name;
    let speed = &train.speed;
    let type_name = &train.type_;
    println!(
        "Es fährt ein Zug namens {} ein und er hat folgenden Typ {}. Er kommt mit {} km/h eingefahren",
        name, type_name, speed
    );
}
```
Great! The function is much slimmer and we have moved the logic to a more appropriate location. But can we do even better?

17. Create an implementation of Display on the Train struct
```rust
impl Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Es fährt ein Zug namens {} ein und er hat folgenden Typ {}. Er kommt mit {} km/h eingefahren",
        self.name, self.type_, self.speed)
    }
}
```

18. Now we just need to use this method and get rid of the printer function
```rust
fn main() {
    let train = Train {
        name: String::from("Thomas"),
        speed: 99.0,
        type_: TrainType::Regional(String::from("S-7")),
    };
    print!("{}", train);
}
```
Awesome! We just learned how to implement Traits from the standard library to streamline our code and move logic into implementation blocks of our data structures.


## Cargo

Let's see why cargo is awesome!

### Adding packages

1. First we inspect the Cargo.toml file
2. Now let's add a package:
```shell
cargo add serde_json
```
3. Let's use that package to transform some json. First we need to get some json data:
```shell
curl https://jsonplaceholder.typicode.com/posts/1 > post.json
```
4. Now we can use the serde package to parse the json. Let's uncomment the train for now.

```rust
// let train = Train {
//     name: String::from("Thomas"),
//     speed: 99.0,
//     type_: TrainType::Regional(String::from("S-7")),
// };
// print!("{}", train);
```

5. We can now open the file that we saved as post.json in our main function:
```rust
let mut file = File::open("post.json").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();

println!("{}", contents);
```

Let's check this code out for a minute. What are these .unwrap()'s for? Also let's run the code!

**Does it compile?**

Observe the error message. We did not import the necessary packages. But the compiler is very helpful and tells us what to do.

```rust
use std::fs::File;
```

Let's run the code again!

**Does it compile?**

Observe the error message. We did not import the necessary packages, again! But the compiler is very helpful and tells us what to do.

```rust
use std::io::Read;
```

Let's give this another try!

**Does it compile?**

6. Since we can see that we have successfully read the file, let's try to parse the json with serde_json
```rust
use serde_json::Value;

let post: Value = serde_json::from_str(&contents).unwrap();
```

Do not forget to remove the following line or the borrow checker will complain:
```rust
println!("{}", contents);
```

7. Now we can access the fields of the json object
```rust
let title = post["title"].as_str().unwrap();
let body = post["body"].as_str().unwrap();
println!("Title: {}", title);
println!("Body: {}", body);
```

Nice! We can access the values like on a hashmap. But what if we want to access the values in a more type safe way?

8. Let's create a struct that represents the json object. First we need to add the core serde package:
```shell
cargo add serde
```

To use the feature we need from serde, we need to add a feature flag to our Cargo.toml:
```toml
[dependencies]
serde = { version = "1.0.215", features = ["derive"] }
```

Now we can use its derive macro to create a struct that represents the json object:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}
```

9. Now we can parse the json object into our struct
```rust
let post: Post = serde_json::from_str(&contents).unwrap();

println!("Post: {:?}", post);
```

**Does it compile?**

As you can see, this does not compile. Why?

We need to derive Debug to print the struct. Let's do that!

```rust
#[derive(Deserialize, Debug)]
```

Great! Now check out the code we used to get there. As you can see, we explicitly told the compiler to use Post on the left side of the equals sign. This is needed to apply the correct transformation. This is a very powerful feature of Rust, as it allows us to be very explicit about what we want to do.


## Option and Result

Two of the most powerful types in Rust are Option and Result. Let's see how they work!

10. Let's create a function that returns an Option
```rust
fn get_post() -> Option<Post> {
    let post = Post {
        userId: 1,
        id: 1,
        title: "foo".into(),
        body: "bar".into(),
    };
    Some(post)
}
```

Let's uncomment the json parsing for now:
```rust
    // let mut file = File::open("post.json").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();

    // let post: Post = serde_json::from_str(&contents).unwrap();

    // println!("Post: {:?}", post);
```

Now we can use the Option in the main function
```rust
let post = get_post().unwrap();
```

**Does it compile?**

As you can see, there is no problem with this code.
But what happens if the Option is None?
Let's change it!

```rust
fn get_post() -> Option<Post> {
    None
}
```

**Does it compile?**

It does! But why? Isnt Rust memory safe?
So why does a runtime error occur?
This is where .unwrap() comes into play. It is a method that is available on the Option type. It will panic if the Option is None, which we do not want.

11. Let's change the code to handle the None case
```rust
match get_post() {
    Some(post) => println!("Post found: {:?}", post),
    None => println!("No post found"),
};
```

**Does it compile?**

Great! Now we have a way to handle the None case. But what if we want to return an error instead of None?

12. Let's change the function so that it returns a Result
```rust
fn get_post() -> Result<Post, String> {
    Err("Strike! Post office not operational!".into())
}
```

**Does it compile?**

As you can see, the compiler is not happy with this code. Why? We did not adjust for the change from Option to Result. Let's do that!

```rust
match get_post() {
    Ok(post) => println!("Post found: {:?}", post),
    Err(message) => println!("No post found: {}", message),
};
```

Since we are responsible developers, let's write some tests!

## Running tests
13. Let's create a test 
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_post() {
        let post = get_post().unwrap();
        assert_eq!(post.userId, 1);
    }
}
```

Observe the structure. We have several elements:
- the test macro
- the module declaration with "mod tests"
- the use statement to import the function we want to test
- the actual test function and its test decorator (macro)

14. Now we can run the tests
```shell
cargo test
```

Observe the output. The test fails. Let's adjust the function to make it pass.

```rust
fn get_post() -> Result<Post, String> {
    Ok(Post {
        userId: 1,
        id: 1,
        title: "foo".into(),
        body: "bar".into(),
    })
}
```

Run the test again, as you can see it passes now.

## Creating the documentation

15. In order to create the documentation, we need to add some comments to our code. Let's do that!

```rust
/// This is the main function
```

```rust
/// Here we get a post
/// # Returns
/// A post
/// # Errors
/// If the post office is not operational
```

16. Now we can create the documentation
```shell
cargo doc --open
```

As you can see Rust is awesome, we can create documentation for our whole codebase with a single command.

## Webserver

But wait, there is more. Let's create a webserver!
We can use the famous axum crate for this.

17. First we need to add the axum crate to our dependencies
```shell
cargo add axum
cargo add tokio
```

18. Also, make sure you enable the feature flag for the tokio runtime in your Cargo.toml
```toml
tokio = { version = "1.43.0", features = ["rt-multi-thread", "macros"] }
```

19. Now we can create a webserver

Import the necessary packages

```rust
use axum::{
    routing::get,
    Router,
    Json,
};
```

Decorate the main function with the tokio runtime

```rust
#[tokio::main]
async fn main() {}
```

Create a router and start a server

```rust
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
```

When opening http://localhost:3000 in your browser, you should see the message "Hello, World!". Congratulations, you just created a webserver with Rust!

Let's now return a JSON of our Post instead. First we create a new handler for this purpose:

```rust
    async fn post_handler() -> Json<Post> {
        let post = get_post().unwrap();
        Json(post)
    }
```

Then we return it as json:

```rust
    let app = Router::new().route("/", get(post_handler));
```

## Throw it into a Docker container

20. First we need to create a Dockerfile

```Dockerfile
FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/trains"]
```

21. Now we can build the Docker image

```shell
docker build -t trains .
```

22. And run the container

```shell
docker run -p 3000:3000 trains
```

Now you can open http://localhost:3000 in your browser and see JSON Output!
