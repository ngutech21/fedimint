use fedimint_api::{db::Database, module::registry::ModuleDecoderRegistry};
use fedimint_rocksdb::RocksDb;
use fedimint_storage::{config::StorageConfig, StorageModule};
use uuid::Uuid;

fn open_temp_db(temp_path: &str) -> RocksDb {
    let path = tempfile::Builder::new()
        .prefix(temp_path)
        .tempdir()
        .expect("Could not create temp dir");

    RocksDb::open(path).expect("Could not open RocksDB")
}

#[tokio::test]
async fn test_store_and_retrieve() -> anyhow::Result<()> {
    let db = Database::new(
        open_temp_db("store_and_retrieve_test"),
        ModuleDecoderRegistry::default(),
    );
    let module = StorageModule {
        cfg: StorageConfig::default(),
    };
    let value = "Hello, world!".to_string();
    let key = Uuid::new_v4().hyphenated().to_string();

    let mut tx = db.begin_transaction().await;
    // store string in storage-module
    let store_result = module.store(&mut tx, key, value.clone()).await?;
    tx.commit_tx().await.expect("DB Error");

    // retrieve string from storage-module
    let mut tx_ret = db.begin_transaction().await;
    let retrieve_result = module.retrieve(&mut tx_ret, store_result.0).await;
    tx_ret.commit_tx().await.expect("DB Error");
    assert_eq!(value, retrieve_result?);
    Ok(())
}
