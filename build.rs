use std::env;

use pyo3_build_config::pyo3_build_script_impl::{cargo_env_var, errors::Result};
use pyo3_build_config::{bail, print_feature_cfgs, InterpreterConfig};

fn ensure_auto_initialize_ok(interpreter_config: &InterpreterConfig) -> Result<()> {
    if cargo_env_var("CARGO_FEATURE_AUTO_INITIALIZE").is_some() && !interpreter_config.shared {
        bail!(
            "The `auto-initialize` feature is enabled, but your python installation only supports \
            embedding the Python interpreter statically. If you are attempting to run tests, or a \
            binary which is okay to link dynamically, install a Python distribution which ships \
            with the Python shared library.\n\
            \n\
            Embedding the Python interpreter statically does not yet have first-class support in \
            PyO3. If you are sure you intend to do this, disable the `auto-initialize` feature.\n\
            \n\
            For more information, see \
            https://pyo3.rs/v{pyo3_version}/\
                building_and_distribution.html#embedding-python-in-rust",
            pyo3_version = env::var("CARGO_PKG_VERSION").unwrap()
        );
    }
    Ok(())
}

/// Prepares the PyO3 crate for compilation.
///
/// This loads the config from pyo3-build-config and then makes some additional checks to improve UX
/// for users.
///
/// Emits the cargo configuration based on this config as well as a few checks of the Rust compiler
/// version to enable features which aren't supported on MSRV.
fn configure_pyo3() -> Result<()> {
    let interpreter_config = pyo3_build_config::get();

    interpreter_config.emit_pyo3_cfgs();

    ensure_auto_initialize_ok(interpreter_config)?;

    // Emit cfgs like `thread_local_const_init`
    print_feature_cfgs();

    Ok(())
}

fn main() {
    if let Err(e) = configure_pyo3() {
        eprintln!("error: {}", e.report());
        std::process::exit(1)
    }
}
