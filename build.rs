use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/sparkplug_b.proto"], &["src/"])?;
    Ok(())
}
