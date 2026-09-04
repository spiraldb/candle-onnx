use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[allow(clippy::doc_overindented_list_items)]");
    config.compile_protos(&["src/onnx.proto3"], &["src/"])?;
    Ok(())
}
