//! SigmaOS-efi-boot-services — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_efi_boot_services_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-efi-boot-services: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-efi-boot-services: ok");
}
