//! astrobase-server database statistics.

use tokio::sync::RwLock;

/// Represents the statistics.
pub struct Stats {
    number_of_records: RwLock<usize>,
    get_ok_fail: RwLock<(usize, usize)>,
    insert_ok_fail: RwLock<(usize, usize)>,
    delete_ok_fail: RwLock<(usize, usize)>,
    update_ok_fail: RwLock<(usize, usize)>,
}

impl Stats {
    /// Constructs new instance of Stats.
    pub fn new() -> Self {
        Stats {
            number_of_records: RwLock::new(0),
            get_ok_fail: RwLock::new((0, 0)),
            insert_ok_fail: RwLock::new((0, 0)),
            delete_ok_fail: RwLock::new((0, 0)),
            update_ok_fail: RwLock::new((0, 0)),
        }
    }

    /// Updates the GET stats.
    pub async fn get_ok(&self, ok: bool) {
        let mut ok_fail = self.get_ok_fail.write().await;
        if ok {
            ok_fail.0 += 1
        } else {
            ok_fail.1 += 1
        }
    }

    /// Updates the INSERT stats (may increment number of records).
    pub async fn insert_ok(&self, ok: bool) {
        let mut ok_fail = self.insert_ok_fail.write().await;
        if ok {
            let mut count = self.number_of_records.write().await;
            *count += 1;
            ok_fail.0 += 1
        } else {
            ok_fail.1 += 1
        }
    }

    /// Updates the DELETE stats (may decrement number of records).
    pub async fn delete_ok(&self, ok: bool) {
        let mut ok_fail = self.delete_ok_fail.write().await;
        if ok {
            let mut count = self.number_of_records.write().await;
            *count -= 1;
            ok_fail.0 += 1
        } else {
            ok_fail.1 += 1
        }
    }

    /// Updates the UPDATE stats.
    pub async fn update_ok(&self, ok: bool) {
        let mut ok_fail = self.update_ok_fail.write().await;
        if ok {
            ok_fail.0 += 1
        } else {
            ok_fail.1 += 1
        }
    }

    /// Dumps the data to stderr.
    pub async fn dump(&self) {
        let n = { self.number_of_records.read().await };
        let get = { self.get_ok_fail.read().await };
        let ins = { self.insert_ok_fail.read().await };
        let del = { self.delete_ok_fail.read().await };
        let upd = { self.update_ok_fail.read().await };
        eprintln!("NR: {},    GET(ok/fail): {:?},    INSERT(ok/fail): {:?},    DELETE(ok/fail): {:?},    UPDATE(ok/fail): {:?}",
                  n, get, ins, del, upd);
    }
}
