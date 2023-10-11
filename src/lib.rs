#![cfg_attr(DIAGNOSTICS, feature(proc_macro_diagnostic))]

pub fn warn(_msg: &str) {
    #[cfg(DIAGNOSTICS)]
    proc_macro::Diagnostic::spanned(
        [proc_macro::Span::call_site()].as_ref(),
        proc_macro::Level::Warning,
        _msg,
    )
    .emit();
    #[cfg(not(DIAGNOSTICS))]
    std::fs::write("warnings", "flipflopflap").unwrap();
}

pub fn warn_build() {
    if let Ok(warnings) = std::fs::read_to_string("warnings") {
        for warning in warnings.split('\n') {
            println!("cargo:warning={}", warning);
        }
    }
    // if let Ok(v) = rustc_version::version_meta() {
    //     if v.channel == rustc_version::Channel::Nightly {
    //         println!("cargo:rustc-cfg=DIAGNOSTICS");
    //     }
    // }
}
