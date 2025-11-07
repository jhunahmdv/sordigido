use autocxx::prelude::*;

include_cpp! {
    #include "/home/ahmdv/projects/rust/sordigido/src/cpp/include/bridge_digidoc.h"
    safety!(unsafe)
    generate!("digidoc::appInfo_shim")
    generate!("digidoc::version_shim")
    generate!("digidoc::create_signer_shim")
}

fn main() {
        let info = ffi::digidoc::appInfo_shim();
        println!("app_name: {:?}", info);

        let version = ffi::digidoc::version_shim();
        println!("version: {:?}", version);

        let signer = ffi::digidoc::create_signer_shim();
        println!("signer: {:?}", signer);
    
}
