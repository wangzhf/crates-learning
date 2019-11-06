use rocksdb::{Options, DB};

fn main() {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_max_open_files(1000);
    opts.set_use_fsync(true);

    let path = "./storage";
    // let db = DB::open_default("./storage").unwrap();
    let db = DB::open(&opts, path).unwrap();

    db.put(b"my key", b"my value");
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operation problem encountered: {}", e),
    }

    db.delete(b"my key").unwrap();
}
