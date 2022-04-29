use std::path::PathBuf;

pub fn get_resource(resource_name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let resource_path = root.join(resource_name);
    resource_path
}
