#include "includes/qiniu_sdk.h"
#include <array>
#include <map>
#include <vector>
#include <iostream>

namespace qiniu_sdk
{
    namespace utils
    {
        std::vector<std::array<rust::Str, 2>> convert_headers(const std::map<std::string, std::string> &headers) noexcept
        {
            std::vector<std::array<rust::Str, 2>> header_vec(headers.size());
            size_t idx = 0;
            for (const auto pair : headers)
            {
                const std::string &header_name = pair.first;
                const std::string &header_value = pair.second;
                const std::array<rust::Str, 2> new_pair = {rust::Str(header_name), rust::Str(header_value)};
                header_vec[idx++] = new_pair;
            }
            return header_vec;
        }

        std::vector<std::string> from_rust_string_slice_to_cpp_string_vector(const rust::String *str, size_t str_count) noexcept
        {
            std::vector<std::string> string_vec(str_count);
            for (size_t i = 0; i < str_count; i++)
            {
                string_vec[i] = std::string(str[i]);
            }
            return string_vec;
        }

        std::vector<rust::Str> from_rust_string_slice_to_rust_str_vector(const std::string *str, size_t str_count) noexcept
        {
            std::vector<rust::Str> str_vec(str_count);
            for (size_t i = 0; i < str_count; i++)
            {
                str_vec[i] = rust::Str(str[i]);
            }
            return str_vec;
        }
    }
}
