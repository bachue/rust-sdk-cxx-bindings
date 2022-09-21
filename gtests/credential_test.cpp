#include "gtest/gtest.h"
#include "includes/qiniu_sdk.h"
#include <fstream>
#include <sstream>

class CredentialTest : public ::testing::Test
{
protected:
    void TearDown() override
    {
        qiniu_sdk::credential::GlobalCredentialProvider::clear();
        qiniu_sdk::credential::EnvCredentialProvider::clear();
    }
};

TEST_F(CredentialTest, CredentialTest)
{
    qiniu_sdk::credential::Credential credential("abcdefghklmnopq", "1234567890");

    EXPECT_EQ("abcdefghklmnopq", credential.get_access_key());
    EXPECT_EQ("1234567890", credential.get_secret_key());

    {
        qiniu_sdk::credential::Credential cloned = credential;
        EXPECT_EQ("abcdefghklmnopq", cloned.get_access_key());
        EXPECT_EQ("1234567890", cloned.get_secret_key());
    }

    {
        const char *data = "hello";
        EXPECT_EQ(credential.sign(data, strlen(data)), "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0=");
        EXPECT_EQ(credential.sign_with_data(data, strlen(data)), "abcdefghklmnopq:BZYt5uVRy1RVt5ZTXbaIt2ROVMA=:aGVsbG8=");
    }

    {
        std::istringstream stream("hello");
        EXPECT_EQ(credential.sign_reader(&stream), "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0=");
    }

    {
        std::ifstream fstream("not_existed_file", std::ios::binary | std::ios::in);
        EXPECT_ANY_THROW({ credential.sign_reader(&fstream); });
        fstream.close();
    }

    {
        const char *data = "name=test&language=go";
        auto authorization = credential.authorization_v1_for_request("http://upload.qiniup.com/", "application/x-www-form-urlencoded", data, strlen(data));
        EXPECT_EQ(authorization, "QBox abcdefghklmnopq:VlWNSauF13XCI1YGoeGMUC229lI=");
    }

    {
        std::istringstream stream("name=test&language=go");
        auto authorization = credential.authorization_v1_for_request_with_body_reader("http://upload.qiniup.com/", "application/x-www-form-urlencoded", &stream);
    }

    {
        auto download_url = credential.sign_download_url("http://www.qiniu.com/?go=1", std::chrono::hours(1));
        EXPECT_EQ(download_url.rfind("http://www.qiniu.com/?go=1&e=", 0), 0);
        EXPECT_NE(download_url.rfind("&token=abcdefghklmnopq"), std::string::npos);
    }
}

TEST_F(CredentialTest, GlobalCredentialProviderTest)
{
    qiniu_sdk::credential::Credential credential("global_ak", "global_sk");
    qiniu_sdk::credential::GlobalCredentialProvider::setup(credential);
    qiniu_sdk::credential::GlobalCredentialProvider gcp;
    qiniu_sdk::credential::Credential c = gcp.get();
    EXPECT_EQ("global_ak", c.get_access_key());
    EXPECT_EQ("global_sk", c.get_secret_key());
}

TEST_F(CredentialTest, EnvCredentialProviderTest)
{
    qiniu_sdk::credential::Credential credential("env_ak", "env_sk");
    qiniu_sdk::credential::EnvCredentialProvider::setup(credential);
    qiniu_sdk::credential::EnvCredentialProvider ecp;
    qiniu_sdk::credential::Credential c = ecp.get();
    EXPECT_EQ("env_ak", c.get_access_key());
    EXPECT_EQ("env_sk", c.get_secret_key());
}

TEST_F(CredentialTest, ChainCredentialsProviderTest)
{
    qiniu_sdk::credential::GlobalCredentialProvider gcp;
    qiniu_sdk::credential::EnvCredentialProvider ecp;
    qiniu_sdk::credential::Credential c("real_ak", "real_sk");

    auto builder = qiniu_sdk::credential::ChainCredentialsProviderBuilder(std::move(gcp));
    builder.append_credential(std::move(ecp));
    builder.append_credential(std::move(c));
    auto chain = builder.build();

    {
        qiniu_sdk::credential::Credential r = chain.get();
        EXPECT_EQ("real_ak", r.get_access_key());
        EXPECT_EQ("real_sk", r.get_secret_key());
    }

    {
        qiniu_sdk::credential::Credential new_env_cred("env_ak", "env_sk");
        qiniu_sdk::credential::EnvCredentialProvider::setup(new_env_cred);
    }

    {
        qiniu_sdk::credential::Credential r = chain.get();
        EXPECT_EQ("env_ak", r.get_access_key());
        EXPECT_EQ("env_sk", r.get_secret_key());
    }

    {
        qiniu_sdk::credential::Credential new_glb_cred("global_ak", "global_sk");
        qiniu_sdk::credential::GlobalCredentialProvider::setup(new_glb_cred);
    }

    {
        qiniu_sdk::credential::Credential r = chain.get();
        EXPECT_EQ("global_ak", r.get_access_key());
        EXPECT_EQ("global_sk", r.get_secret_key());
    }
}
