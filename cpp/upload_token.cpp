#include "includes/qiniu_bindings.h"
#include <vector>
#include <iostream>

namespace qiniu_bindings
{
    namespace utils
    {
        std::vector<rust::Str> from_rust_string_slice_to_rust_str_vector(const std::string *str, size_t str_count) noexcept;
        std::vector<std::string> from_rust_string_slice_to_cpp_string_vector(const rust::String *str, size_t str_count) noexcept;
    }

    namespace upload_token
    {

        UploadPolicyBuilder::UploadPolicyBuilder(const UploadPolicyBuilder &builder) noexcept : inner(_internal::rust_sdk_ffi::upload_policy_builder_clone(*builder.inner))
        {
        }

        UploadPolicyBuilder::UploadPolicyBuilder(rust::Box<_internal::rust_sdk_ffi::UploadPolicyBuilder> &&builder) noexcept : inner(std::move(builder))
        {
        }

        const _internal::rust_sdk_ffi::UploadPolicyBuilder &UploadPolicyBuilder::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::UploadPolicyBuilder &UploadPolicyBuilder::operator*() noexcept
        {
            return *this->inner;
        }

        UploadPolicyBuilder UploadPolicyBuilder::new_for_bucket(const std::string &bucket, const std::chrono::nanoseconds &lifetime) noexcept
        {
            return UploadPolicyBuilder(_internal::rust_sdk_ffi::new_upload_policy_for_bucket(bucket, lifetime.count()));
        }

        UploadPolicyBuilder UploadPolicyBuilder::new_for_object(const std::string &bucket, const std::string &object, const std::chrono::nanoseconds &lifetime) noexcept
        {
            return UploadPolicyBuilder(_internal::rust_sdk_ffi::new_upload_policy_for_object(bucket, object, lifetime.count()));
        }

        UploadPolicyBuilder UploadPolicyBuilder::new_for_objects_with_prefix(const std::string &bucket, const std::string &prefix, const std::chrono::nanoseconds &lifetime) noexcept
        {
            return UploadPolicyBuilder(_internal::rust_sdk_ffi::new_upload_policy_for_objects_with_prefix(bucket, prefix, lifetime.count()));
        }

        UploadPolicyBuilder &UploadPolicyBuilder::token_lifetime(const std::chrono::nanoseconds &lifetime) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_token_lifetime(*this->inner, lifetime.count());
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::token_deadline(const std::chrono::nanoseconds &time_since_epoch) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_token_deadline(*this->inner, time_since_epoch.count());
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::insert_only() noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_insert_only(*this->inner);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::enable_mime_detection() noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_enable_mime_detection(*this->inner);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::disable_mime_detection() noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_disable_mime_detection(*this->inner);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::file_type(uint8_t file_type) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_file_type(*this->inner, file_type);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::return_url(const std::string &url) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_return_url(*this->inner, url);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::return_body(const std::string &body) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_return_body(*this->inner, body);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::callback(const std::string *urls, const size_t urls_count, const std::string &host, const std::string &body, const std::string &body_type) noexcept
        {
            auto urls_vector = utils::from_rust_string_slice_to_rust_str_vector(urls, urls_count);
            auto urls_slice = rust::Slice<const rust::Str>(urls_vector.data(), urls_count);
            _internal::rust_sdk_ffi::upload_policy_builder_set_callback(*this->inner, urls_slice, host, body, body_type);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::save_as(const std::string &save_as, bool force) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_save_as(*this->inner, save_as, force);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::file_size_limitation(uint64_t min_file_size, uint64_t max_file_size) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_file_size_limitation(*this->inner, min_file_size, max_file_size);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::mime_types(const std::string *mime_types, const size_t mime_types_count) noexcept
        {
            auto mime_types_vector = utils::from_rust_string_slice_to_rust_str_vector(mime_types, mime_types_count);
            auto mime_types_slice = rust::Slice<const rust::Str>(mime_types_vector.data(), mime_types_count);
            _internal::rust_sdk_ffi::upload_policy_builder_set_mime_types(*this->inner, mime_types_slice);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::object_lifetime(const std::chrono::nanoseconds &lifetime) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_object_lifetime(*this->inner, lifetime.count());
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::set_as_string(const std::string &key, const std::string &value) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_string(*this->inner, key, value);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::set_as_integer(const std::string &key, int64_t value) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_integer(*this->inner, key, value);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::set_as_bool(const std::string &key, bool value) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_set_bool(*this->inner, key, value);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::unset(const std::string &key) noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_unset(*this->inner, key);
            return *this;
        }

        UploadPolicyBuilder &UploadPolicyBuilder::reset() noexcept
        {
            _internal::rust_sdk_ffi::upload_policy_builder_reset(*this->inner);
            return *this;
        }

        UploadPolicy UploadPolicyBuilder::build() const noexcept
        {
            return UploadPolicy(*this);
        }

        UploadPolicy::UploadPolicy(const UploadPolicy &policy) noexcept : inner(_internal::rust_sdk_ffi::upload_policy_clone(*policy))
        {
        }

        UploadPolicy::UploadPolicy(const UploadPolicyBuilder &builder) noexcept : inner(_internal::rust_sdk_ffi::upload_policy_builder_build(*builder))
        {
        }

        UploadPolicy::UploadPolicy(rust::Box<_internal::rust_sdk_ffi::UploadPolicy> &&builder) noexcept : inner(std::move(builder))
        {
        }

        UploadPolicy UploadPolicy::from_json(const std::string &json)
        {
            return UploadPolicy(_internal::rust_sdk_ffi::upload_policy_from_json(json));
        }

        const _internal::rust_sdk_ffi::UploadPolicy &UploadPolicy::operator*() const noexcept
        {
            return *this->inner;
        }

        _internal::rust_sdk_ffi::UploadPolicy &UploadPolicy::operator*() noexcept
        {
            return *this->inner;
        }

        std::string UploadPolicy::bucket() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_bucket(**this));
        }

        std::string UploadPolicy::key() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_key(**this));
        }

        bool UploadPolicy::has_prefixal_object_key() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_has_prefixal_object_key(**this);
        }

        bool UploadPolicy::is_insert_only() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_insert_only(**this);
        }

        bool UploadPolicy::is_mime_detection_enabled() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_mime_detection_enabled(**this);
        }

        std::chrono::nanoseconds UploadPolicy::token_deadline() const
        {
            uint64_t seconds = _internal::rust_sdk_ffi::upload_policy_get_token_deadline(**this);
            return std::chrono::seconds(seconds);
        }

        std::string UploadPolicy::return_url() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_return_url(**this));
        }

        std::string UploadPolicy::return_body() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_return_body(**this));
        }

        std::vector<std::string> UploadPolicy::callback_urls() const noexcept
        {
            auto urls = _internal::rust_sdk_ffi::upload_policy_get_callback_urls(**this);
            return utils::from_rust_string_slice_to_cpp_string_vector(&urls[0], urls.size());
        }

        std::string UploadPolicy::callback_host() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_callback_host(**this));
        }

        std::string UploadPolicy::callback_body() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_callback_body(**this));
        }

        std::string UploadPolicy::callback_body_type() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_callback_body_type(**this));
        }

        std::string UploadPolicy::save_key() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_save_key(**this));
        }

        bool UploadPolicy::is_save_key_forced() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_save_key_forced(**this);
        }

        uint64_t UploadPolicy::max_file_size() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_get_max_file_size(**this);
        }

        uint64_t UploadPolicy::min_file_size() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_get_min_file_size(**this);
        }

        std::vector<std::string> UploadPolicy::mime_types() const noexcept
        {
            auto types = _internal::rust_sdk_ffi::upload_policy_get_mime_types(**this);
            return utils::from_rust_string_slice_to_cpp_string_vector(types.data(), types.size());
        }

        uint8_t UploadPolicy::file_type() const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_get_file_type(**this);
        }

        std::chrono::nanoseconds UploadPolicy::object_lifetime() const noexcept
        {
            uint64_t seconds = _internal::rust_sdk_ffi::upload_policy_get_object_lifetime(**this);
            return std::chrono::seconds(seconds);
        }

        std::string UploadPolicy::to_json() const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_as_json(**this));
        }

        bool UploadPolicy::has_key(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_has_key(**this, key);
        }

        std::string UploadPolicy::get_as_string(const std::string &key) const noexcept
        {
            return std::string(_internal::rust_sdk_ffi::upload_policy_get_string(**this, key));
        }

        int64_t UploadPolicy::get_as_integer(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_get_integer(**this, key);
        }

        bool UploadPolicy::get_as_bool(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_get_bool(**this, key);
        }

        bool UploadPolicy::is_string(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_string(**this, key);
        }

        bool UploadPolicy::is_integer(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_integer(**this, key);
        }

        bool UploadPolicy::is_bool(const std::string &key) const noexcept
        {
            return _internal::rust_sdk_ffi::upload_policy_is_bool(**this, key);
        }
    }
}
