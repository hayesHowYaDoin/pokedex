// use std::sync::LazyLock;

// #[cfg(target_os = "linux")]
// static DATABASE_PATH: LazyLock<String> = LazyLock::new(||{
//     match std::env::var("POKEDEX_DATA_PATH").ok() {
//         Some(p) => p.trim_end_matches('/').to_string(),
//         None => "/usr/share/".to_string(),
//     }
// });

// #[cfg(target_os = "macos")]
// pub const CONFIG_PATH: &str = "/usr/local/etc/ebbflow";
// #[cfg(windows)]
// lazy_static! {
//     pub static ref CONFIG_PATH: String = { "\\Program Files\\ebbflow".to_string() };
// }
