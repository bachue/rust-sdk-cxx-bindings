#include "gtest/gtest.h"
#include "includes/qiniu_bindings.h"

TEST(EtagTest, EtagTest)
{
    {
        std::istringstream empty("");
        EXPECT_EQ(qiniu_bindings::etag::etag_of(&empty), "Fto5o-5ea0sNMlW_75VgGJCv2AcJ");
    }

    {
        std::istringstream ss("etag");
        EXPECT_EQ(qiniu_bindings::etag::etag_of(&ss), "FpLiADEaVoALPkdb8tJEJyRTXoe_");
    }
}

TEST(EtagTest, EtagV1Test)
{
    qiniu_bindings::etag::EtagV1 v1;

    const char *data = "etag";
    v1.update(data, strlen(data));

    char etag_result[qiniu_bindings::etag::ETAG_SIZE + 1] = {0};
    v1.finalize(etag_result);
    EXPECT_STREQ(etag_result, "FpLiADEaVoALPkdb8tJEJyRTXoe_");
}
