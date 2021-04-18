//! astrobase-server key-value database unit tests.

use super::{Database, InMemory, Persistent};

#[tokio::test]
async fn inmemory() {
    let db = populate_database::<InMemory>().await;
    run_tests(db).await;
}

#[tokio::test]
async fn persistent() {
    let db = populate_database::<Persistent>().await;
    run_tests(db).await;
}

async fn populate_database<Db: Database>() -> Db {
    let db = Db::new();
    db.clear().await.ok();
    db.insert("a", "1").await.ok();
    db.insert("b", "2").await.ok();
    db.insert("c", "3").await.ok();
    db.insert("d", "4").await.ok();
    db
}

async fn run_tests<Db: Database>(db: Db) {
    test_get(&db).await;
    test_insert(&db).await;
    test_delete(&db).await;
    test_update(&db).await;
}

async fn test_get<Db: Database>(db: &Db) {
    let r = db.get("a").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "1");

    let r = db.get("z").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' is missing");
}

async fn test_insert<Db: Database>(db: &Db) {
    let r = db.insert("z", "26").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "");

    let r = db.get("z").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "26");

    let r = db.insert("z", "26").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' already exists");

    let r = db.delete("z").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "26");

    let r = db.insert("z", "1000").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "");

    let r = db.get("z").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "1000");
}

async fn test_delete<Db: Database>(db: &Db) {
    let r = db.delete("d").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "4");

    let r = db.delete("z").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "1000");

    let r = db.delete("z").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' is missing already");

    let r = db.delete("d").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'd' is missing already");
}

async fn test_update<Db: Database>(db: &Db) {
    let r = db.update("a", "100").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "");

    let r = db.get("a").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "100");

    let r = db.update("b", "2").await;
    assert!(r.is_err());
    assert_eq!(
        r.unwrap_err().to_string(),
        "Record 'b' already exists and identical"
    );

    let r = db.update("z", "26").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' is missing");

    let r = db.delete("a").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "100");

    let r = db.update("a", "100").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'a' is missing");
}
