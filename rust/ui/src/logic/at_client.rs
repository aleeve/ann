use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn getActor(s: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn getFollows(s: &str) -> Result<JsValue, JsValue>;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    pub did: String,
    pub texts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Follows {
    pub did: String,
    pub follows: Vec<String>,
}


#[derive(Debug)]
pub enum Rating {
    More,
    Ok,
    Indifferent,
    Meh,
    Yikes,
}

pub async fn get_actor(did: &str) -> Actor {
    let res = getActor(did).await.unwrap();
    let actor: Actor = serde_wasm_bindgen::from_value(res).unwrap();
    actor
}

pub async fn get_follows(did: &str) -> Follows {
    let res = getFollows(did).await.unwrap();
    let follows: Follows = serde_wasm_bindgen::from_value(res).unwrap();
    follows
}


