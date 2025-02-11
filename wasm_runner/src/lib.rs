use anyhow::Context;
use anyhow::Result;
use handler::FactorServer;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use wasm_states::WasmStates;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

mod wasm_states;

mod handler {
    wasmtime::component::bindgen!({
        trappable_imports: true,
        path: "../wit",
        world: "factor-server",
        //concurrent_imports: true,
        //concurrent_exports: true,
        async: true,
    });
}

impl handler::huygens::service::host::Host for wasm_states::WasmStates {
    async fn redis_get(&mut self, s: String) -> wasmtime::Result<String> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(format!("{s} - entered host - exited host"))
    }
}

#[derive(Clone)]
pub struct WasmRuntime {
    engine: Arc<Engine>,
    store: Arc<Mutex<Store<wasm_states::WasmStates>>>,
    instance: Arc<Mutex<Option<FactorServer>>>,
}

impl WasmRuntime {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.wasm_component_model_async(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let wasm_states = WasmStates::new();
        let store = Store::new(&engine, wasm_states);

        Ok(Self {
            engine: Arc::new(engine),
            store: Arc::new(Mutex::new(store)),
            instance: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn load_wasm(&mut self, wasm_path: &str) -> Result<()> {
        let mut store = self.store.lock().await;
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        handler::FactorServer::add_to_linker(&mut linker, |ctx| ctx)?;
        let component = Component::from_file(&self.engine, wasm_path)?;
        println!("component new done");

        let instance = handler::FactorServer::instantiate_async(&mut *store, &component, &linker)
            .await
            .context("failed to init the world")?;

        println!("instantiate_async done");
        let res = instance
            .huygens_service_guest()
            .call_factor_get(&mut *store, "test")
            .await?;
        println!("call guest done");
        println!("{:?}", res);

        //println!("instantiate_async done");
        let mut ref_instance = self.instance.lock().await;
        *ref_instance = Some(instance);

        Ok(())
    }

    pub async fn execute_factor(&self, _func_name: &str, param: i32) -> Result<String> {
        println!("execute factor begin");
        let mut store = self.store.lock().await;
        let mut instance = self.instance.lock().await;
        println!("execute call_factor_get");
        let res = instance
            .as_mut()
            .unwrap()
            .huygens_service_guest()
            .call_factor_get(&mut *store, format!("{param}").as_str())
            .await?;
        println!("{:?}", res);
        Ok("exec done".to_string())
    }
}
