mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "factor-server",
        async: true,
    });

    use super::Component;
    export!(Component);
}

//use std::fmt::format;

use bindings::exports::huygens::service::guest::Guest;
//use bindings::huygens::service::host;

struct Component;

impl Guest for Component {
    async fn factor_get(s: String) -> String {
        println!("guest begin");
        format!("hello guest: {}", s)
        //format!(
        //    "{} - exited guest",
        //    host::redis_get(&format!("{s} - entered guest")).await
        //)
    }
}
