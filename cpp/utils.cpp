#include "includes/qiniu_sdk.h"
#include <array>
#include <map>
#include <vector>

namespace qiniu_sdk
{
    namespace credential
    {
        std::vector<std::array<rust::Str, 2>> convert_headers(const std::map<std::string, std::string> &headers) noexcept
        {
            std::vector<std::array<rust::Str, 2>> header_vec(headers.size());
            for (const auto pair : headers)
            {
                const std::string &header_name = pair.first;
                const std::string &header_value = pair.second;
                const std::array<rust::Str, 2> new_pair = {rust::Str(header_name), rust::Str(header_value)};
                header_vec.push_back(new_pair);
            }
            return header_vec;
        }
    }
}
