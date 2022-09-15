#include <string>
#include "rust/cxx.h"
#include "qiniu-sdk-ffi/src/lib.rs.h"

#ifndef ___QINIU_SDK_DEFINED
#define ___QINIU_SDK_DEFINED

namespace qiniu_sdk
{
    void initialize();

    namespace credential
    {
        class Credential
        {
        public:
            Credential(const std::string &, const std::string &);
            std::string get_access_key() const;
            std::string get_secret_key() const;
            std::string sign(const uint8_t *, size_t) const;
            std::string sign_with_data(const uint8_t *, size_t) const;
            std::string sign_reader(std::istream *stream) const;

        private:
            ::rust::Box<::qiniu_sdk_ffi::rust::Credential> credential;
        };
    }
}

#endif // ___QINIU_SDK_DEFINED
