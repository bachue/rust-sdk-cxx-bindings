#include "includes/qiniu_sdk.h"
#include <type_traits>

namespace qiniu_sdk
{
    namespace credential
    {
        Credential::Credential(const std::string &access_key, const std::string &secret_key) : credential(::qiniu_sdk_ffi::rust::new_credential(::rust::Str(access_key), ::rust::Str(secret_key)))
        {
        }

        std::string Credential::get_access_key() const
        {
            return std::string(::qiniu_sdk_ffi::rust::credential_get_access_key(*this->credential));
        }

        std::string Credential::get_secret_key() const
        {
            return std::string(::qiniu_sdk_ffi::rust::credential_get_secret_key(*this->credential));
        }

        std::string Credential::sign(const uint8_t *data, size_t len) const
        {
            return std::string(::qiniu_sdk_ffi::rust::credential_sign(*this->credential, ::rust::Slice<const uint8_t>(data, len)));
        }

        std::string Credential::sign_with_data(const uint8_t *data, size_t len) const
        {
            return std::string(::qiniu_sdk_ffi::rust::credential_sign_with_data(*this->credential, ::rust::Slice<const uint8_t>(data, len)));
        }

        std::string Credential::sign_reader(std::istream *stream) const
        {
            return std::string(::qiniu_sdk_ffi::rust::credential_sign_reader(*this->credential, static_cast<void *>(stream)));
        }
    }
}
