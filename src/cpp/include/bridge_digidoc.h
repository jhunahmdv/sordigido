#pragma once
#include <string>
#include "Container.h"   // from libdigidocpp
#include <crypto/Signer.h> // from libdigidocpp

namespace digidoc {
    // Expose a simple wrapper for Rust
    std::string appInfo_shim();
    std::string version_shim();
    Signer* create_signer_shim(); // Placeholder for creating a Signer instance
}
