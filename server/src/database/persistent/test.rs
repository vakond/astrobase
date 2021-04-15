//! astrobase-server persistent key-value database unit tests.
//! Note: we use serial-test crate to avoid concurrent access to a single storage file.

use super::Persistent;
use crate::database::Database;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn get() {
    let db = populate_database().await;

    let r = db.get("a").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "1");

    let r = db.get("z").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' is missing");
}

#[tokio::test]
#[serial]
async fn insert() {
    let db = populate_database().await;

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

#[tokio::test]
#[serial]
async fn delete() {
    let db = populate_database().await;

    let r = db.delete("d").await;
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), "4");

    let r = db.delete("z").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'z' is missing already");

    let r = db.delete("d").await;
    assert!(r.is_err());
    assert_eq!(r.unwrap_err().to_string(), "Record 'd' is missing already");
}

#[tokio::test]
#[serial]
async fn update() {
    let db = populate_database().await;

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

/// Create a database from scrutch.
async fn populate_database() -> Persistent {
    let db = Persistent::new();
    db.clear().await.ok();
    db.insert("a", "1").await.ok();
    db.insert("b", "2").await.ok();
    db.insert("c", "3").await.ok();
    db.insert("d", "4").await.ok();
    db
}
