use env_logger::Env;

#[allow(dead_code)]
pub fn init_test_log() {
    let _ = env_logger::builder()
        .is_test(true)
        .parse_env(Env::default().default_filter_or(log::Level::Debug.as_str()))
        .format_timestamp(None)
        .try_init();
}
