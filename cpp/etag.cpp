#include "includes/qiniu_bindings.h"

namespace qiniu_bindings
{
    namespace etag
    {
        std::string etag_of(std::istream *data_reader)
        {
            return std::string(_internal::rust_sdk_ffi::etag_of(static_cast<void *>(data_reader)));
        }

        void etag_to_buf(std::istream *data_reader, uint8_t *buf)
        {
            _internal::rust_sdk_ffi::etag_to_buf(static_cast<void *>(data_reader), static_cast<void *>(buf));
        }

        EtagV1::EtagV1() noexcept : inner(_internal::rust_sdk_ffi::new_etag_v1())
        {
        }

        EtagV1::EtagV1(EtagV1 &&other) noexcept : inner(std::move(other.inner))
        {
        }

        void EtagV1::update(const char *data, size_t len) noexcept
        {
            auto data_slice = rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t *>(data), len);
            _internal::rust_sdk_ffi::etag_v1_update(*this->inner, data_slice);
        }

        void EtagV1::finalize(char *buf) noexcept
        {
            _internal::rust_sdk_ffi::etag_v1_finalize_to_buf(*this->inner, static_cast<void *>(buf));
        }

        void EtagV1::reset() noexcept
        {
            _internal::rust_sdk_ffi::etag_v1_reset(*this->inner);
        }
    }
}
