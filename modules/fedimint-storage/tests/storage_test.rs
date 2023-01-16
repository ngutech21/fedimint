use fedimint_api::{db::Database, module::registry::ModuleDecoderRegistry};
use fedimint_rocksdb::RocksDb;
use fedimint_storage::{
    config::{StorageConfig, StorageConfigConsensus, StorageConfigPrivate},
    StorageModule,
};
use serde_json::json;

fn open_temp_db(temp_path: &str) -> RocksDb {
    let path = tempfile::Builder::new()
        .prefix(temp_path)
        .tempdir()
        .unwrap();

    RocksDb::open(path).unwrap()
}

#[test]
fn json_test() {
    println!("Hello, world!");
    let _v = json!("a string");
    //let result = serde_json::from_str::<serde_json::Value>(r#" {"param": "1"} "#).unwrap();
    let result = serde_json::from_str::<serde_json::Value>(r#" 1  "#).unwrap();
    println!("result: {:?}", result);
}

#[test_log::test(tokio::test)]
async fn store_and_retrieve_test() {
    let rocks_db = open_temp_db("store_and_retrieve_test");

    let cfg = StorageConfig {
        private: StorageConfigPrivate {
            something_private: 42,
        },
        consensus: StorageConfigConsensus { something: 108 },
    };

    let mdr = ModuleDecoderRegistry::default();
    let db = Database::new(rocks_db, mdr);
    let mut tx = db.begin_transaction().await;

    let module = StorageModule { cfg };
    let value = "Hello, world!".to_string();
    let key = uuid::Uuid::new_v4().hyphenated().to_string();
    let store_result = module.store(&mut tx, key, value).await.unwrap();
    tx.commit_tx().await.expect("DB Error");
    dbg!(&store_result);

    let mut tx_ret = db.begin_transaction().await;
    let retrieve_result = module.retrieve(&mut tx_ret, store_result.0).await;
    tx_ret.commit_tx().await.expect("DB Error");
    dbg!(&retrieve_result);
    assert_eq!("Hello, world!".to_string(), retrieve_result.unwrap())
}
