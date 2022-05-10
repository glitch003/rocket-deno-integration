use anyhow::Result;
use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use std::convert::TryFrom;
use ureq;

// fn to do an http get
#[op]
fn op_http_get(url: String) -> Result<String, deno_core::error::AnyError> {
    println!("http_get({})", url);

    let body = ureq::get(&url).call()?.into_string()?;
    println!("got body in rust {}", body);

    Ok(body)
}

pub fn execute_js() -> Result<()> {
    println!("JS code execution started at {}", chrono::Local::now());

    // Build a deno_core::Extension providing custom ops
    let ext = Extension::builder()
        .ops(vec![
            // An op for getting an HTTP url
            // The op-layer automatically deserializes inputs
            // and serializes the returned Result & value
            op_http_get::decl(),
        ])
        .build();

    // Initialize a runtime instance
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    let code_to_run = r#"
      const console = {
        log: function (value) {
          Deno.core.print(value.toString() + "\n");
        },
      };
      
      const go = async () => {
        const resp = Deno.core.opSync(
          "op_http_get",
          "https://ipfs.litgateway.com/ipfs/QmNiDrDnTiSo4y78qKwaZboq8KfT9Y3CRrnM7pfUmG1EFq"
        );
        console.log("resp: " + resp);
      };
      
      go();
    "#;

    js_runtime
        .execute_script("serverless_function.js", &code_to_run)
        .unwrap();

    //   js_runtime.run_event_loop(false).await;

    println!("JS code execution finished at {}", chrono::Local::now());

    Ok(())
}
