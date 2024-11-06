use std::sync::Arc;

use gloo::utils::format::JsValueSerdeExt;
use gloo_console::{log, warn};
use leptos::*;
use idb::{Database as IdbcDatabase, DatabaseEvent, Error, Factory, IndexParams, KeyPath, ObjectStoreParams, TransactionMode};
use serde_json::{Value, json};
use wasm_bindgen::JsValue;

const DB_NAME: &str = "training";
const VERSION: Option<u32> = Some(1);

#[derive(Debug, Clone)]
pub struct Database {
    db: Arc<IdbcDatabase>,
    name: String,
    version: u32

}

impl Database {
    pub async fn new(stores: Vec<String>) -> Result<Database, Error>{
        let db = create_database(stores).await?;
        let mdb = Database {
            db: Arc::new(db),
            name: "Training".into(),
            version: 1,
        };
        Ok(mdb)
    }

    pub async fn get_data(&self ,store:&str, hash: &str) -> Result<Value, Error> {
        get_data(&self.db, store, hash).await
    }

    pub async fn add_data(&self, store: &str, value: &Value) -> Result<JsValue, Error> {
        add_data(&self.db, store, value).await
    }

}

async fn create_database(stores: Vec<String>) -> Result<IdbcDatabase, Error> {
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

fn add_objectstore(database: &IdbcDatabase, name: &str, index: &str) -> Result<(), Error>{
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


async fn add_data(database: &IdbcDatabase, store: &str, value: &Value) -> Result<JsValue, Error> {
    let mut data = value.clone();

    // Create a read-write transaction
    let transaction = database.transaction(&[store], TransactionMode::ReadWrite)?;

    // Get the object store
    let store = transaction.object_store(store).unwrap();
    gloo_console::log!(value.to_string());
    let index = store.index("hash").expect("The hash index should have been created at start");

    // Check if data already exists
    let hash = JsValue::from_serde(&value["hash"]).unwrap();
    if let Some(val) = index.get(hash).unwrap().await.unwrap() {
        if let Ok(val) = serde_wasm_bindgen::from_value::<Value>(val){
            let key = val["id"].as_i64().unwrap();
            data.as_object_mut().unwrap().insert("id".to_string(), key.into());
        }
    };
    

    // Add data to object store
    let id = store
        .put(
            &JsValue::from_serde(&data).unwrap(),
            None
        )
        .unwrap()
        .await.unwrap();

    // Commit the transaction
    transaction.commit()?.await?;

    Ok(id)
}

async fn get_data(database: &IdbcDatabase, store:&str, hash: &str) -> Result<Value, Error> {
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
