use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(0).unwrap();

    let keys = icu_datagen::keys_from_bin(&path).unwrap();
    for key in keys {
        println!("{}", key.path().get());
    }
}
