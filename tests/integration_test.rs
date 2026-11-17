//! Integration tests for SigmaOS-efi-boot-services.

use sigmaos_efi_boot_services_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
