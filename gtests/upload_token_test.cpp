#include "gtest/gtest.h"
#include "includes/qiniu_sdk.h"
#include <chrono>
#include <array>

TEST(UploadTokenTest, UploadPolicyTest)
{
    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_bucket("test-bucket", std::chrono::hours(1));
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "");
        EXPECT_FALSE(policy.has_prefixal_object_key());

        auto deadline = std::chrono::time_point<std::chrono::system_clock, std::chrono::nanoseconds>(policy.token_deadline());
        auto now = std::chrono::system_clock::now();
        auto diff = std::chrono::duration_cast<std::chrono::seconds>(deadline - now);
        EXPECT_NEAR(diff.count(), 3600, 5);
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_object("test-bucket", "test-key", std::chrono::hours(1));
        builder.insert_only().enable_mime_detection().file_type(qiniu_sdk::upload_token::INFREQUENT_ACCESS);
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        EXPECT_FALSE(policy.has_prefixal_object_key());
        EXPECT_FALSE(policy.has_key("isPrefixalScope"));
        EXPECT_TRUE(policy.is_insert_only());
        EXPECT_EQ(policy.get_as_integer("insertOnly"), 1);
        EXPECT_TRUE(policy.is_mime_detection_enabled());
        EXPECT_EQ(policy.get_as_integer("detectMime"), 1);
        EXPECT_EQ(policy.file_type(), qiniu_sdk::upload_token::INFREQUENT_ACCESS);
        EXPECT_EQ(policy.get_as_integer("fileType"), qiniu_sdk::upload_token::INFREQUENT_ACCESS);

        auto deadline = std::chrono::time_point<std::chrono::system_clock, std::chrono::nanoseconds>(policy.token_deadline());
        auto now = std::chrono::system_clock::now();
        auto diff = std::chrono::duration_cast<std::chrono::seconds>(deadline - now);
        EXPECT_NEAR(diff.count(), 3600, 5);
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_objects_with_prefix("test-bucket", "test-key", std::chrono::hours(1));
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        EXPECT_TRUE(policy.has_prefixal_object_key());
        EXPECT_EQ(policy.get_as_integer("isPrefixalScope"), 1);

        auto deadline = std::chrono::time_point<std::chrono::system_clock, std::chrono::nanoseconds>(policy.token_deadline());
        auto now = std::chrono::system_clock::now();
        auto diff = std::chrono::duration_cast<std::chrono::seconds>(deadline - now);
        EXPECT_NEAR(diff.count(), 3600, 5);
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_object("test-bucket", "test-key", std::chrono::hours(1));
        builder.return_url("http://return.qiniu.com").return_body("name=$(fname)&size=$(fsize)");
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        EXPECT_EQ(policy.return_url(), "http://return.qiniu.com");
        EXPECT_EQ(policy.return_body(), "name=$(fname)&size=$(fsize)");
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_object("test-bucket", "test-key", std::chrono::hours(1));
        std::array<std::string, 2> urls{"http://callback1.qiniu.com", "http://callback2.qiniu.com"};
        builder.callback(urls.data(), urls.size(), "callback.qiniu.com", "name=$(fname)&size=$(fsize)", "application/x-www-form-urlencoded");
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        auto callback_urls = policy.callback_urls();
        EXPECT_TRUE(std::equal(urls.cbegin(), urls.cend(), callback_urls.cbegin()));
        EXPECT_EQ(policy.callback_host(), "callback.qiniu.com");
        EXPECT_EQ(policy.callback_body(), "name=$(fname)&size=$(fsize)");
        EXPECT_EQ(policy.callback_body_type(), "application/x-www-form-urlencoded");
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_object("test-bucket", "test-key", std::chrono::hours(1));
        std::array<std::string, 3> mime_types{"application/x-www-form-urlencoded", "application/json", "application/csv"};
        builder.mime_types(mime_types.data(), mime_types.size());
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        auto got_mime_types = policy.mime_types();
        EXPECT_TRUE(std::equal(mime_types.cbegin(), mime_types.cend(), got_mime_types.cbegin()));
        EXPECT_EQ(policy.get_as_string("mimeLimit"), "application/x-www-form-urlencoded;application/json;application/csv");
    }

    {
        auto builder = qiniu_sdk::upload_token::UploadPolicyBuilder::new_for_object("test-bucket", "test-key", std::chrono::hours(1));
        builder.set_as_string("testString", "stringValue").set_as_integer("testNumber", 1).set_as_bool("testBoolean", true);
        auto policy = builder.build();
        EXPECT_EQ(policy.bucket(), "test-bucket");
        EXPECT_EQ(policy.key(), "test-key");
        EXPECT_TRUE(policy.has_key("testString"));
        EXPECT_TRUE(policy.has_key("testNumber"));
        EXPECT_TRUE(policy.has_key("testBoolean"));
        EXPECT_TRUE(policy.is_string("testString"));
        EXPECT_TRUE(policy.is_integer("testNumber"));
        EXPECT_TRUE(policy.is_bool("testBoolean"));
        EXPECT_EQ(policy.get_as_string("testString"), "stringValue");
        EXPECT_EQ(policy.get_as_integer("testNumber"), 1);
        EXPECT_TRUE(policy.get_as_bool("testBoolean"));
    }
}
