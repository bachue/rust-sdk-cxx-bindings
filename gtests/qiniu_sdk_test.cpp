#include <gtest/gtest.h>
#include <includes/qiniu_sdk.h>

TEST(QiniuSDKTest, BasicAssertions)
{
    qiniu_sdk::initialize();
    std::cout << "Done" << std::endl;
}
