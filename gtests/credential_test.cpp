#include <gtest/gtest.h>
#include <includes/qiniu_sdk.h>
#include <fstream>
#include <sstream>

TEST(QiniuSDKTest, CredentialTest)
{
    qiniu_sdk::credential::Credential credential("abcdefghklmnopq", "1234567890");

    EXPECT_EQ("abcdefghklmnopq", credential.get_access_key());
    EXPECT_EQ("1234567890", credential.get_secret_key());
    const uint8_t data[] = "hello";
    EXPECT_EQ(credential.sign(&data[0], sizeof(data) / sizeof(uint8_t) - 1), "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0=");
    EXPECT_EQ(credential.sign_with_data(&data[0], sizeof(data) / sizeof(uint8_t) - 1), "abcdefghklmnopq:BZYt5uVRy1RVt5ZTXbaIt2ROVMA=:aGVsbG8=");

    std::istringstream stream("hello");
    EXPECT_EQ(credential.sign_reader(&stream), "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0=");

    std::ifstream fstream("not_existed_file", std::ios::binary | std::ios::in);
    EXPECT_ANY_THROW({ credential.sign_reader(&fstream); });
    fstream.close();
}
