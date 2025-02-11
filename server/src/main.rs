use wasm_runner::WasmRuntime;

use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut wasm_runtime = WasmRuntime::new()?;
    let cwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .display()
        .to_string();
    wasm_runtime
        .load_wasm(format!("{}/../{}", cwd, "wasm_modules/guest.wasm").as_str())
        .await?;
    println!("load wasm done");
    Ok(())
}
