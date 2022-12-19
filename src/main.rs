fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::trace!("some trace log");
    log::debug!("some debug log");
    log::info!("some info log");
    log::warn!("some warn log");
    log::error!("some error log");

    perform_wasm::build_perform!(namespace, i32, String);
}
