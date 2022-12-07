mod db_config;
fn main() {
    let path = "src/data/config.json".to_string();
    let config = db_config::read_config(path);
    println!("{:#?}", config);
}
