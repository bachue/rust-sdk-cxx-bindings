#include <string>
#include <map>
#include <chrono>
#include "rust/cxx.h"
#include "qiniu-sdk-ffi/src/lib.rs.h"

#ifndef ___QINIU_SDK_DEFINED
#define ___QINIU_SDK_DEFINED

namespace qiniu_sdk
{
    namespace _internal
    {
        int initialize();
    }

    namespace credential
    {
        class GotCredential;
        class CredentialProvider;
        class GlobalCredentialProvider;

        class Credential final
        {
        public:
            Credential(const std::string &, const std::string &) noexcept;
            Credential(const Credential &) noexcept;
            Credential(const GotCredential &) noexcept;
            Credential(rust::Box<_internal::rust_sdk_ffi::Credential> &&credential) noexcept;
            const _internal::rust_sdk_ffi::Credential &operator*() const noexcept;
            _internal::rust_sdk_ffi::Credential &operator*() noexcept;
            std::string get_access_key() const noexcept;
            std::string get_secret_key() const noexcept;
            std::string sign(const uint8_t *, size_t) const;
            std::string sign_with_data(const uint8_t *, size_t) const;
            std::string sign_reader(std::istream *) const;
            std::string authorization_v1_for_request(const std::string &, const std::string &, const uint8_t *, size_t) const;
            std::string authorization_v1_for_request_with_body_reader(const std::string &, const std::string &, std::istream *) const;
            std::string authorization_v2_for_request(const std::string &, const std::string &, const std::map<std::string, std::string> &, const uint8_t *, size_t) const;
            std::string authorization_v2_for_request_with_body_reader(const std::string &, const std::string &, const std::map<std::string, std::string> &, std::istream *) const;
            std::string sign_download_url(const std::string &, const std::chrono::nanoseconds &) const;

        private:
            rust::Box<_internal::rust_sdk_ffi::Credential> inner;
            friend class CredentialProvider;
        };

        class GotCredential final
        {
        public:
            GotCredential(rust::Box<_internal::rust_sdk_ffi::GotCredential> &&got_credential) noexcept;
            const _internal::rust_sdk_ffi::GotCredential &operator*() const noexcept;
            _internal::rust_sdk_ffi::GotCredential &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GotCredential> inner;
        };

        class GetCredentialOptions final
        {
        public:
            GetCredentialOptions() noexcept;
            GetCredentialOptions(const GetCredentialOptions &) noexcept;
            GetCredentialOptions(rust::Box<_internal::rust_sdk_ffi::GetCredentialOptions> &&options) noexcept;
            const _internal::rust_sdk_ffi::GetCredentialOptions &operator*() const noexcept;
            _internal::rust_sdk_ffi::GetCredentialOptions &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GetCredentialOptions> inner;
        };

        class CredentialProvider
        {
        public:
            CredentialProvider() = delete;
            CredentialProvider(const CredentialProvider &) noexcept;
            CredentialProvider(Credential) noexcept;
            CredentialProvider(rust::Box<_internal::rust_sdk_ffi::CredentialProvider> &&provider) noexcept;
            const _internal::rust_sdk_ffi::CredentialProvider &operator*() const noexcept;
            _internal::rust_sdk_ffi::CredentialProvider &operator*() noexcept;
            virtual GotCredential get(const GetCredentialOptions &options = GetCredentialOptions()) const;

        protected:
            rust::Box<_internal::rust_sdk_ffi::CredentialProvider> inner;
            friend class ChainCredentialsProviderBuilder;
        };

        class GlobalCredentialProvider final : public CredentialProvider
        {
        public:
            GlobalCredentialProvider() noexcept;
            static void setup(const Credential &credential) noexcept;
            static void clear() noexcept;
        };

        class EnvCredentialProvider final : public CredentialProvider
        {
        public:
            EnvCredentialProvider() noexcept;
            static void setup(const Credential &credential) noexcept;
            static void clear() noexcept;
        };

        class ChainCredentialsProvider;

        class ChainCredentialsProviderBuilder final
        {
        public:
            ChainCredentialsProviderBuilder() = delete;
            ChainCredentialsProviderBuilder(CredentialProvider provider) noexcept;
            ChainCredentialsProviderBuilder(rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> &&provider) noexcept;
            const _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() const noexcept;
            _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() noexcept;
            void append_credential(CredentialProvider provider) noexcept;
            void prepend_credential(CredentialProvider provider) noexcept;
            ChainCredentialsProvider build() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> inner;
        };

        class ChainCredentialsProvider final : public CredentialProvider
        {
        public:
            ChainCredentialsProvider() = delete;
            ChainCredentialsProvider(ChainCredentialsProviderBuilder &) noexcept;
        };
    }

    static int _ = _internal::initialize();
}

#endif // ___QINIU_SDK_DEFINED
