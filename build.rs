use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all("src/protos")?;
    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["protos"])
        .input("protos/bean.proto")
        .out_dir("src/protos")
        .run()?;
    Ok(())
}
