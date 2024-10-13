use gloo_console::{log,warn};
use leptos::*;
use idb::{Database, DatabaseEvent, Error, Factory, IndexParams, KeyPath, ObjectStoreParams, Request, TransactionMode};
use serde::Serialize;
use serde_json::{Value, json};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

const DB_NAME: &str = "training";
const VERSION: Option<u32> = Some(1);

async fn create_database(stores: Vec<String>) -> Result<Database, Error> {
    let factory = Factory::new()?;
    let mut open_request = factory.open(DB_NAME, VERSION).unwrap();
    open_request.on_upgrade_needed(|event| {
        match event.new_version() {
            Ok(VERSION) =>{
                log!("Setting up object stores for {VERSION:?}");
                let database = event.database().unwrap();
                for store in stores{
                    add_objectstore(&database, &store, "hash").unwrap();
                }
            },
            _ => warn!("Ohno")
        };
    });

    open_request.await
}

fn add_objectstore(database: &Database, name: &str, index: &str) -> Result<(), Error>{
    let mut store_params = ObjectStoreParams::new();
    store_params.auto_increment(true);
    store_params.key_path(Some(KeyPath::new_single("id")));

    // Create object store
    let store = database
        .create_object_store(name, store_params)
        .unwrap();

    // Prepare index params
    let mut index_params = IndexParams::new();
    index_params.unique(true);

    // Create index on object store
    store
        .create_index(index, KeyPath::new_single(index), Some(index_params))
        .unwrap();
    Ok(())

}


async fn add_data(database: &Database, store: &str, value: &mut Value) -> Result<JsValue, Error> {
    // Create a read-write transaction
    let transaction = database.transaction(&[store], TransactionMode::ReadWrite)?;

    // Get the object store
    let store = transaction.object_store(store).unwrap();
    let hash = value.get("hash").unwrap().as_str().unwrap();
    let v = store.index("hash")?.get(JsValue::from(hash)).unwrap().await.unwrap();
    if v.is_some() {
        let v = serde_wasm_bindgen::from_value::<Value>(v.unwrap()).ok();
        match v {
            Some(v) => {value.as_object_mut().unwrap().insert("id".to_string(), v.get("id").unwrap().to_owned());},
            None => ()
        }
    };

    // Add data to object store
    let id = store
        .put(
            &value.serialize(&Serializer::json_compatible()).unwrap(),
            None
        )
        .unwrap()
        .await.unwrap();

    // Commit the transaction
    transaction.commit()?.await?;

    Ok(id)
}

async fn get_data(database: &Database, store:&str, hash: &str) -> Result<Value, Error> {
    // Create a read-only transaction
    let transaction = database
        .transaction(&[store], TransactionMode::ReadOnly)
        .unwrap();

    // Get the object store
    let store = transaction.object_store(store).unwrap();

    // Get the stored data
    let stored_employee: JsValue = store.index("hash")?.get(JsValue::from(hash))?.await.unwrap().unwrap();
    log!(&stored_employee);

    // Deserialize the stored data
    let stored_employee: Value = serde_wasm_bindgen::from_value(stored_employee).unwrap();

    // Wait for the transaction to complete (alternatively, you can also commit the transaction)
    transaction.await?;

    Ok(stored_employee)
}

pub async fn hej(hash: String) -> Value{
    let db = create_database(vec!["data".to_string(), "models".to_string()]).await.unwrap();
    let mut data = json!({
        "hash": hash,
        "hopp": 123
    });

    log!("Adding data");
    add_data(&db, "data", &mut data).await;
    log!("Getting data");
    get_data(&db, "data", hash.as_str()).await.unwrap()
}
