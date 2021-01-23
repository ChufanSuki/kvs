use clap::clap_app;

fn main() {
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

    match matches.subcommand_name() {
        Some("set") => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Some("get") => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Some("rm") => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        _ => {
            eprintln!("unknown command");
            std::process::exit(1);
        }
    }
}
