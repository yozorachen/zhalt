pub fn show_help(target_env_info: &str) {
    let msg = format!(r#"zhalt for {}"#, target_env_info);
    println!("{}", msg);
}
