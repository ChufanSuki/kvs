use clap::clap_app;
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
    let matches = clap_app!(kvs =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Chufan Chen. <allenplato28@gmail.com>")
        (about: "A key-value store cli")
        (@subcommand set =>
            (about: "set key-value pair")
            (@arg KEY: +required "key")
            (@arg VALUE: +required "value")
        )
       (@subcommand get =>
            (about: "get key-value pair by key")
            (@arg KEY: +required "key")
        )
       (@subcommand rm =>
            (about: "remove key-value pair by key")
            (@arg KEY: +required "key")
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("set", Some(set_matches)) => {
            let key = set_matches.value_of("KEY").expect("KEY argument missing");
            let value = set_matches
                .value_of("VALUE")
                .expect("VALUE argument missing");
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(get_matches)) => {
            let key = get_matches.value_of("KEY").expect("KEY argument missing");
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(rm_matches)) => {
            let key = rm_matches.value_of("KEY").expect("KEY argument missing");
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
