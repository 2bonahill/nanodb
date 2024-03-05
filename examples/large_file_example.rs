use nanodb::nanodb::NanoDB;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let db = NanoDB::open("examples/data/large-file.json").unwrap();

    let _number_of_actors = db.data().await.unwrap().len().unwrap();

    let actor: Actor = db
        .data()
        .await
        .unwrap()
        .at(0)
        .unwrap()
        .get("actor")
        .unwrap()
        .into()
        .unwrap();

    // dbg!(actor.url);
}

#[derive(Debug, serde::Deserialize)]
struct Actor {
    avatar_url: String,
    gravatar_id: String,
    id: i64,
    login: String,
    url: String,
}
