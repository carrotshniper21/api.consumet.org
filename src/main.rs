use consumet_api_rs::providers::movies;

#[derive(Debug)]
struct Test {
    id: String,
}

#[tokio::main]
async fn main() {
    let flixhq = movies::FlixHQ;

    let deez = flixhq.search("hi", None).await.unwrap();
    let result = Test {
        id: deez.results[0].id.clone().unwrap(),
    };

    println!("{:#?}", result);
}
