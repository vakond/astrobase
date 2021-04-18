//! astrobase-server database statistics.

/// Represents the statistics.
#[derive(Default)]
pub struct Stats {
    number_of_records: usize,
    get_ok_fail: (usize, usize),
    insert_ok_fail: (usize, usize),
    delete_ok_fail: (usize, usize),
    update_ok_fail: (usize, usize),
}

impl Stats {
    /// Updates the GET stats.
    pub fn get(&mut self, ok: bool) {
        if ok {
            self.get_ok_fail.0 += 1
        } else {
            self.get_ok_fail.1 += 1
        }
    }

    /// Updates the INSERT stats (may increment number of records).
    pub fn insert(&mut self, ok: bool) {
        if ok {
            self.number_of_records += 1;
            self.insert_ok_fail.0 += 1
        } else {
            self.insert_ok_fail.1 += 1
        }
    }

    /// Updates the DELETE stats (may decrement number of records).
    pub fn delete(&mut self, ok: bool) {
        if ok {
            self.number_of_records -= 1;
            self.delete_ok_fail.0 += 1
        } else {
            self.delete_ok_fail.1 += 1
        }
    }

    /// Updates the UPDATE stats.
    pub fn update(&mut self, ok: bool) {
        if ok {
            self.update_ok_fail.0 += 1
        } else {
            self.update_ok_fail.1 += 1
        }
    }

    /// Dumps the data to stderr.
    pub fn dump(&self) {
        let n = self.number_of_records;
        let get = self.get_ok_fail;
        let ins = self.insert_ok_fail;
        let del = self.delete_ok_fail;
        let upd = self.update_ok_fail;
        eprintln!("NR: {},    GET(ok/fail): {:?},    INSERT(ok/fail): {:?},    DELETE(ok/fail): {:?},    UPDATE(ok/fail): {:?}",
                  n, get, ins, del, upd);
    }
}
