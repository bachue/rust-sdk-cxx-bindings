#include "includes/qiniu_sdk.h"
#include <type_traits>
#include <array>
#include <map>
#include <vector>

namespace qiniu_sdk
{
    namespace credential
    {
        std::vector<std::array<rust::Str, 2>> convert_headers(const std::map<std::string, std::string> &) noexcept;

        Credential::Credential(const std::string &access_key, const std::string &secret_key) noexcept : inner(_internal::rust_sdk_ffi::new_credential(rust::Str(access_key), rust::Str(secret_key)))
        {
        }

        Credential::Credential(const Credential &credential) noexcept : inner(_internal::rust_sdk_ffi::credential_clone(*credential))
        {
        }

        Credential::Credential(const GotCredential &credential) noexcept : inner(_internal::rust_sdk_ffi::got_credential_get_credential(*credential))
        {
        }

        Credential::Credential(rust::Box<_internal::rust_sdk_ffi::Credential> &&credential) noexcept : inner(std::move(credential))
        {
        }

        const _internal::rust_sdk_ffi::Credential &Credential::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::Credential &Credential::operator*() noexcept
        {
            return *this->inner;
        }

        std::string Credential::get_access_key() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::credential_get_access_key(*this->inner));
        }

        std::string Credential::get_secret_key() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::credential_get_secret_key(*this->inner));
        }

        std::string Credential::sign(const uint8_t *data, size_t len) const
        {
            auto data_slice = rust::Slice<const uint8_t>(data, len);
            return std::string(_internal::rust_sdk_ffi::credential_sign(*this->inner, data_slice));
        }

        std::string Credential::sign_with_data(const uint8_t *data, size_t len) const
        {
            auto data_slice = rust::Slice<const uint8_t>(data, len);
            return std::string(_internal::rust_sdk_ffi::credential_sign_with_data(*this->inner, data_slice));
        }

        std::string Credential::sign_reader(std::istream *stream) const
        {
            return std::string(_internal::rust_sdk_ffi::credential_sign_reader(*this->inner, static_cast<void *>(stream)));
        }

        std::string Credential::authorization_v1_for_request(const std::string &url, const std::string &content_type, const uint8_t *body, size_t body_size) const
        {
            auto url_str = rust::Str(url);
            auto content_type_str = rust::Str(content_type);
            auto body_slice = rust::Slice<const uint8_t>(body, body_size);
            return std::string(_internal::rust_sdk_ffi::credential_authorization_v1_for_request(*this->inner, url_str, content_type_str, body_slice));
        }

        std::string Credential::authorization_v1_for_request_with_body_reader(const std::string &url, const std::string &content_type, std::istream *stream) const
        {
            auto url_str = rust::Str(url);
            auto content_type_str = rust::Str(content_type);
            return std::string(_internal::rust_sdk_ffi::credential_authorization_v1_for_request_with_body_reader(*this->inner, url_str, content_type_str, static_cast<void *>(stream)));
        }

        std::string Credential::authorization_v2_for_request(const std::string &method, const std::string &url, const std::map<std::string, std::string> &headers, const uint8_t *body, size_t body_size) const
        {
            auto url_str = rust::Str(url);
            auto method_str = rust::Str(method);
            auto header_vec = convert_headers(headers);
            auto header_slice = rust::Slice<const ::std::array<rust::Str, 2>>(&header_vec[0], header_vec.size());
            auto body_slice = rust::Slice<const uint8_t>(body, body_size);
            return std::string(_internal::rust_sdk_ffi::credential_authorization_v2_for_request(*this->inner, method_str, url_str, header_slice, body_slice));
        }

        std::string Credential::authorization_v2_for_request_with_body_reader(const std::string &method, const std::string &url, const std::map<std::string, std::string> &headers, std::istream *stream) const
        {
            auto url_str = rust::Str(url);
            auto method_str = rust::Str(method);
            auto header_vec = convert_headers(headers);
            auto header_slice = rust::Slice<const ::std::array<rust::Str, 2>>(&header_vec[0], header_vec.size());
            return std::string(_internal::rust_sdk_ffi::credential_authorization_v2_for_request_with_body_reader(*this->inner, method_str, url_str, header_slice, static_cast<void *>(stream)));
        }

        std::string Credential::sign_download_url(const std::string &url, const std::chrono::nanoseconds &lifetime) const
        {
            auto url_str = rust::Str(url);
            return std::string(_internal::rust_sdk_ffi::credential_sign_download_url(*this->inner, url_str, lifetime.count()));
        }

        GotCredential::GotCredential(rust::Box<_internal::rust_sdk_ffi::GotCredential> &&got_credential) noexcept : inner(std::move(got_credential))
        {
        }

        const _internal::rust_sdk_ffi::GotCredential &GotCredential::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::GotCredential &GotCredential::operator*() noexcept
        {
            return *this->inner;
        }

        GetCredentialOptions::GetCredentialOptions() noexcept : inner(_internal::rust_sdk_ffi::new_get_credential_options())
        {
        }

        GetCredentialOptions::GetCredentialOptions(const GetCredentialOptions &options) noexcept : inner(_internal::rust_sdk_ffi::get_credential_options_clone(*options.inner))
        {
        }

        GetCredentialOptions::GetCredentialOptions(rust::Box<_internal::rust_sdk_ffi::GetCredentialOptions> &&options) noexcept : inner(std::move(options))
        {
        }

        const _internal::rust_sdk_ffi::GetCredentialOptions &GetCredentialOptions::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::GetCredentialOptions &GetCredentialOptions::operator*() noexcept
        {
            return *this->inner;
        }

        CredentialProvider::CredentialProvider(Credential credential) noexcept : inner(_internal::rust_sdk_ffi::new_static_credential_provider(std::move(credential.inner)))
        {
        }

        CredentialProvider::CredentialProvider(const CredentialProvider &provider) noexcept : inner(_internal::rust_sdk_ffi::credential_provider_clone(*provider.inner))
        {
        }

        CredentialProvider::CredentialProvider(rust::Box<_internal::rust_sdk_ffi::CredentialProvider> &&provider) noexcept : inner(std::move(provider))
        {
        }

        const _internal::rust_sdk_ffi::CredentialProvider &CredentialProvider::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::CredentialProvider &CredentialProvider::operator*() noexcept
        {
            return *this->inner;
        }

        GotCredential CredentialProvider::get(const GetCredentialOptions &options) const
        {
            return _internal::rust_sdk_ffi::credential_provider_get(*this->inner, *options);
        }

        GlobalCredentialProvider::GlobalCredentialProvider() noexcept : CredentialProvider(_internal::rust_sdk_ffi::new_global_credential_provider())
        {
        }

        void GlobalCredentialProvider::setup(const Credential &credential) noexcept
        {
            _internal::rust_sdk_ffi::global_credential_provider_setup(*credential);
        }

        void GlobalCredentialProvider::clear() noexcept
        {
            _internal::rust_sdk_ffi::global_credential_provider_clear();
        }

        EnvCredentialProvider::EnvCredentialProvider() noexcept : CredentialProvider(_internal::rust_sdk_ffi::new_env_credential_provider())
        {
        }

        void EnvCredentialProvider::setup(const Credential &credential) noexcept
        {
            _internal::rust_sdk_ffi::env_credential_provider_setup(*credential);
        }

        void EnvCredentialProvider::clear() noexcept
        {
            _internal::rust_sdk_ffi::env_credential_provider_clear();
        }

        ChainCredentialsProviderBuilder::ChainCredentialsProviderBuilder(CredentialProvider provider) noexcept : inner(_internal::rust_sdk_ffi::new_chain_credentials_provider_builder(std::move(provider.inner)))
        {
        }

        ChainCredentialsProviderBuilder::ChainCredentialsProviderBuilder(rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> &&provider) noexcept : inner(std::move(provider))
        {
        }

        const _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &ChainCredentialsProviderBuilder::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &ChainCredentialsProviderBuilder::operator*() noexcept
        {
            return *this->inner;
        }

        void ChainCredentialsProviderBuilder::append_credential(CredentialProvider provider) noexcept
        {
            _internal::rust_sdk_ffi::chain_credentials_provider_builder_append_credential(*this->inner, std::move(provider.inner));
        }

        void ChainCredentialsProviderBuilder::prepend_credential(CredentialProvider provider) noexcept
        {
            _internal::rust_sdk_ffi::chain_credentials_provider_builder_prepend_credential(*this->inner, std::move(provider.inner));
        }

        ChainCredentialsProvider ChainCredentialsProviderBuilder::build() noexcept
        {
            return ChainCredentialsProvider(*this);
        }

        ChainCredentialsProvider::ChainCredentialsProvider(ChainCredentialsProviderBuilder &provider) noexcept : CredentialProvider(_internal::rust_sdk_ffi::chain_credentials_provider_builder_build(*provider))
        {
        }
    }
}
