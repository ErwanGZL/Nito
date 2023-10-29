use nito::config::open_config;

fn main() {
    let config = open_config();
    println!("{:#?}", config);
}
