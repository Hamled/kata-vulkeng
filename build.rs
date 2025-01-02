use pkg_config::Config;
fn main() {
    Config::new().probe("xkbcommon").unwrap();
    Config::new().probe("vulkan").unwrap();
}
