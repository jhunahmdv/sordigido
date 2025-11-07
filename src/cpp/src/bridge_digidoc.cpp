#include "bridge_digidoc.h"

namespace digidoc {
    std::string appInfo_shim() {
        return appInfo(); // calls into the actual libdigidocpp.so
    }

    std::string version_shim()
    {
        return version(); // just to use something from libdigidocpp and ensure linkage
    }
    Signer *create_signer_shim()
    {
        return nullptr;
    }
}