use gloo_console::log;
use leptos::*;
use idb::{Database, DatabaseEvent, Error, Factory, IndexParams, KeyPath, ObjectStoreParams,TransactionMode};
use serde::Serialize;
use serde_json::Value;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;


async fn create_database() -> Result<Database, Error> {
    // Get a factory instance from global scope
    let factory = Factory::new()?;

    // Create an open request for the database
    let mut open_request = factory.open("test", Some(1)).unwrap();

    // Add an upgrade handler for database
    open_request.on_upgrade_needed(|event| {
        // Get database instance from event
        let database = event.database().unwrap();

        // Prepare object store params
        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(true);
        store_params.key_path(Some(KeyPath::new_single("id")));

        // Create object store
        let store = database
            .create_object_store("employees", store_params)
            .unwrap();

        // Prepare index params
        let mut index_params = IndexParams::new();

        // Create index on object store
        store
            .create_index("email", KeyPath::new_single("email"), Some(index_params))
            .unwrap();
    });

    // `await` open request
    open_request.await
}


async fn add_data(database: &Database) -> Result<JsValue, Error> {
    // Create a read-write transaction
    let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite)?;

    // Get the object store
    let store = transaction.object_store("employees").unwrap();

    // Prepare data to add
    let employee = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
    });

    // Add data to object store
    let id = store
        .add(
            &employee.serialize(&Serializer::json_compatible()).unwrap(),
            None
        )
        .unwrap()
        .await?;

    // Commit the transaction
    transaction.commit()?.await?;

    Ok(id)
}

async fn get_data(database: &Database, id: JsValue) -> Result<Option<Value>, Error> {
    // Create a read-only transaction
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    // Get the object store
    let store = transaction.object_store("employees").unwrap();

    // Get the stored data
    let stored_employee: Option<JsValue> = store.get(id)?.await?;

    // Deserialize the stored data
    let stored_employee: Option<Value> = stored_employee
        .map(|stored_employee| serde_wasm_bindgen::from_value(stored_employee).unwrap());

    // Wait for the transaction to complete (alternatively, you can also commit the transaction)
    transaction.await?;

    Ok(stored_employee)
}
async fn hej(a:String) -> () {
    let db = create_database().await.unwrap();
    match add_data(&db).await{
        Ok(id) => {
            let e = get_data(&db, id).await;
            log!(format!("{e:?}"));
            log!(a);
        },
        Err(e) => log!(format!("{e:?}"))
    }
    ()
}

#[component]
pub fn StoreData(key: ReadSignal<String>) -> impl IntoView {
    log!("Storing data");
    let db = create_resource(move || key.get(), hej);

    view! {"hej"}
}
