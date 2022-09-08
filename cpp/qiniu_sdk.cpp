#include <string>
#include <memory>
#include <stdexcept>
#include "rust/cxx.h"
#include "qiniu-sdk-ffi/src/lib.rs.h"

namespace qiniu_sdk
{

    template <typename... Args>
    static std::string string_format(const std::string &format, Args... args);

    void initialize()
    {
#if defined(_MSC_VER)
        auto version_info = string_format("MSVC-v%d", _MSC_VER);
#elif defined(__MINGW64__)
        auto version_info = string_format("MinGW-w64-v%d.%d/GCC-v%d.%d.%d", __MINGW64_VERSION_MAJOR, __MINGW64_VERSION_MINOR, __GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__);
#elif defined(__MINGW32__)
        auto version_info = string_format("MinGW32-v%d.%d/GCC-v%d.%d.%d", __MINGW32_MAJOR_VERSION, __MINGW32_MINOR_VERSION, __GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__);
#elif defined(__GNUC__)
        auto version_info = string_format("GCC-v%d.%d.%d", __GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__);
#elif defined(__clang__)
        auto version_info = string_format("Clang-v%s", __clang_version__);
#else
#error Unrecognized Compiler
#endif
        ::qiniu_sdk_ffi::rust::initialize_user_agent(::rust::Str(version_info));
    }

    template <typename... Args>
    std::string string_format(const std::string &format, Args... args)
    {
        int size_s = std::snprintf(nullptr, 0, format.c_str(), args...) + 1; // Extra space for '\0'
        if (size_s <= 0)
        {
            throw std::runtime_error("Error during formatting.");
        }
        auto size = static_cast<size_t>(size_s);
        std::unique_ptr<char[]> buf(new char[size]);
        std::snprintf(buf.get(), size, format.c_str(), args...);
        return std::string(buf.get(), buf.get() + size - 1); // We don't want the '\0' inside
    }

}
