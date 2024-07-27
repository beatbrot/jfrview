use std::future::Future;

#[cfg(target_arch = "wasm32")]
pub fn exec<F>(code: F)
where
    F: Future<Output=()> + 'static
{
    use wasm_bindgen_futures::spawn_local;

    spawn_local(code);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exec<F>(code: F)
where
    F: Future<Output=()> + 'static
{
    use async_std::task;
    task::spawn_local(code);
}