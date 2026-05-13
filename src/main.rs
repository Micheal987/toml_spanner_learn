fn main() {
    println!("{}", "---------------");
    let conf = toml_spanner_learn::core::init_conf();
    println!("conf: {:?}", conf);
    let output = toml_spanner::to_string(&conf).unwrap();
    println!("output: {:?}", output);
    println!("{}", "---------------");
    let from_conf = toml_spanner_learn::from_config::app::Config::new();
    println!("from_conf: {:?}", from_conf);
    let output = toml_spanner::to_string(&from_conf).unwrap();
    println!("output: {:?}", output)
}
