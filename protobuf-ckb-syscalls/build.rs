fn main() -> std::io::Result<()> {
    let mut config = prost_build::Config::new();
    config.btree_map(["."]);
    if std::env::var("CARGO_FEATURE_STD").is_ok() {
        prost_reflect_build::Builder::new()
            .descriptor_pool("crate::generated::traces::DESCRIPTOR_POOL")
            .compile_protos_with_config(config, &["traces.proto"], &[""])?;
    } else {
        config.compile_protos(&["traces.proto"], &[""])?;
    }
    Ok(())
}
