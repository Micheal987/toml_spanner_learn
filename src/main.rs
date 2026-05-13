fn main() {
    println!("{}", "---------------");
    let conf = toml_spanner_learn::core::init_conf();
    println!("conf: {:?}", conf);
    println!("{}", "---------------");
    let from_conf = toml_spanner_learn::from_config::app::Config::new();
    println!("from_conf: {:?}", from_conf);
}
