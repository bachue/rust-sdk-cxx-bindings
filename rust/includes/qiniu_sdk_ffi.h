#include <istream>
#include <memory>
#include "rust/cxx.h"

#ifndef ___QINIU_SDK_FFI_DEFINED
#define ___QINIU_SDK_FFI_DEFINED

namespace qiniu_sdk
{
    namespace _internal
    {
        namespace rust_sdk_ffi
        {

            using c_void = void;

            class SeekableReader
            {
            public:
                SeekableReader() = delete;
                SeekableReader(std::basic_istream<char> *s) : stream(s) {}
                size_t read(uint8_t *data, size_t size, uint8_t *errbit) const;
                uint64_t seek(int64_t off, uint8_t pos, uint8_t *errbit) const;

            private:
                std::basic_istream<char> *stream;
            };
            std::unique_ptr<SeekableReader> new_seekable_reader(void *ptr);
        }
    }
}

#endif // ___QINIU_SDK_FFI_DEFINED
