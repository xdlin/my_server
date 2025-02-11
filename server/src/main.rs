use wasm_runner::WasmRuntime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut wasm_runtime = WasmRuntime::new()?;
    wasm_runtime.load_wasm("/Users/bytedance/src/code.byted.org/bric_huygens/huygens_service/wasm_modules/guest.wasm").await?;
    println!("load wasm done");
    Ok(())
}
