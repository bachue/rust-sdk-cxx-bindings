#include "rust/cxx.h"
#include "qiniu-sdk/src/lib.rs.h"
#include <iostream>

int main()
{
    auto rs = com::qiniu::new_rust_struct(2);
    auto version = com::qiniu::show_version(rs, ::rust::Str("/"));
    std::cout << version.c_str() << std::endl;
    return 0;
}
