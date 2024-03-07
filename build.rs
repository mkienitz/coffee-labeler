use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["protos"])
        .input("protos/bean.proto")
        .out_dir("src/protos")
        .run()?;
    Ok(())
}
