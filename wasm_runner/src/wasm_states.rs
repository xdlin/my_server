use wasmtime::component::ResourceTable;
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

pub struct WasmStates {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl WasmStates {
    pub fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
        Self { table, ctx }
    }
}

impl IoView for WasmStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for WasmStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
