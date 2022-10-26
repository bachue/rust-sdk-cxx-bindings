#include <string>
#include <map>
#include <chrono>
#include <vector>
#include "qiniu-sdk-ffi/src/lib.rs.h"

#ifndef ___QINIU_SDK_DEFINED
#define ___QINIU_SDK_DEFINED

/// @brief  七牛 SDK 名字空间
namespace qiniu_bindings
{
    /// @private
    namespace _internal
    {
        /// @private
        int initialize();
    }

    /// @brief  Etag 名字空间
    namespace etag
    {
        const size_t ETAG_SIZE = 28;
        /// @brief 读取 reader 中的数据并计算它的 Etag V1，生成结果
        /// @param data_reader 数据的输入流
        /// @return 生成的 Etag V1
        std::string etag_of(std::istream *data_reader);

        /// @brief 读取 reader 中的数据并计算它的 Etag V1，生成结果到指定的缓冲中，缓冲区至少保证有 28 个字节
        /// @param data_reader 数据的输入流
        /// @param buf 缓冲区地址，至少有 28 个字节可用
        void etag_to_buf(std::istream *data_reader, char *buf);

        /// @brief  Etag 计算器抽象类
        class Etag
        {
        public:
            /// @brief 向 Etag 计算器写入数据
            /// @param data 要计算的数据地址
            /// @param length 要计算的数据长度，单位为字节
            virtual void update(const char *data, size_t length) noexcept = 0;

            /// @brief 生成结果到指定的缓冲中，缓冲区至少保证有 28 个字节。生成结果后计算器将重置
            /// @param buf 缓冲区地址，至少有 28 个字节可用
            virtual void finalize(char *buf) noexcept = 0;

            /// @brief 重置计算器
            virtual void reset() noexcept = 0;
        };

        class EtagV1 final : public Etag
        {
        public:
            EtagV1() noexcept;
            EtagV1(const EtagV1 &) = delete;
            EtagV1(EtagV1 &&) noexcept;
            /// @brief 向 Etag 计算器写入数据
            /// @param data 要计算的数据地址
            /// @param length 要计算的数据长度，单位为字节
            virtual void update(const char *data, size_t length) noexcept override;

            /// @brief 生成结果到指定的缓冲中，缓冲区至少保证有 28 个字节。生成结果后计算器将重置
            /// @param buf 缓冲区地址，至少有 28 个字节可用
            virtual void finalize(char *buf) noexcept override;

            /// @brief 重置计算器
            virtual void reset() noexcept override;

        private:
            rust::Box<_internal::rust_sdk_ffi::EtagV1> inner;
        };
    }

    namespace upload_token
    {
        class UploadTokenProvider;
        class BucketUploadTokenProvider;
        class ObjectUploadTokenProvider;
    }

    /// @brief  七牛认证信息名字空间
    namespace credential
    {
        class GotCredential;
        class CredentialProvider;

        /// @brief 认证信息
        /// @details 返回认证信息的 AccessKey 和 SecretKey
        class Credential final
        {
        public:
            /// @brief 构造认证信息
            /// @param access_key 七牛 Access Key
            /// @param secret_key 七牛 Secret Key
            Credential(const std::string &access_key, const std::string &secret_key) noexcept;
            /// @brief 复制构造认证信息
            Credential(const Credential &) noexcept;
            /// @brief 将获取的认证信息转换为认证信息
            Credential(const GotCredential &) noexcept;
            /// @private
            Credential(rust::Box<_internal::rust_sdk_ffi::Credential> &&credential) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::Credential &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::Credential &operator*() noexcept;

            /// @brief 获取七牛 Access Key
            /// @return 七牛 Access Key
            std::string get_access_key() const noexcept;

            /// @brief 获取七牛 Secret Key
            /// @return 七牛 Secret Key
            std::string get_secret_key() const noexcept;

            /// @brief  使用七牛签名算法对数据进行签名
            /// @details 参考[管理凭证的签名算法文档](https://developer.qiniu.com/kodo/manual/1201/access-token)
            /// @param data 要签名的数据地址
            /// @param length 要签名的数据长度，单位为字节
            /// @return 签名
            std::string sign(const char *data, size_t length) const;

            /// @brief 使用七牛签名算法对数据进行签名，并同时给出签名和原数据
            /// @details 参考[管理凭证的签名算法文档](https://developer.qiniu.com/kodo/manual/1201/access-token)
            /// @param data 要签名的数据地址
            /// @param length 要签名的数据长度，单位为字节
            /// @return 签名及原数据
            std::string sign_with_data(const char *data, size_t length) const;

            /// @brief 使用七牛签名算法对输入流数据进行签名
            /// @details 参考[管理凭证的签名算法文档](https://developer.qiniu.com/kodo/manual/1201/access-token)
            /// @param data_reader 输入流
            /// @return 签名
            std::string sign_reader(std::istream *data_reader) const;

            /// @brief 使用七牛签名算法 V1 对 HTTP 请求（请求体为内存数据）进行签名，返回 Authorization 的值
            /// @param url 请求的 URL
            /// @param content_type 请求的 Content-Type 信息
            /// @param body 请求 Body 的地址
            /// @param body_size 请求 Body 的长度，单位为字节
            /// @return Authorization 的值
            std::string authorization_v1_for_request(const std::string &url, const std::string &content_type, const char *body, size_t body_size) const;

            /// @brief 使用七牛签名算法 V1 对 HTTP 请求（请求体为输入流）进行签名，返回 Authorization 的值
            /// @param url 请求的 URL
            /// @param content_type 请求的 Content-Type 信息
            /// @param body_reader 请求 Body 的输入流
            /// @return Authorization 的值
            std::string authorization_v1_for_request_with_body_reader(const std::string &url, const std::string &content_type, std::istream *body_reader) const;

            /// @brief  使用七牛签名算法 V2 对 HTTP 请求（请求体为内存数据）进行签名，返回 Authorization 的值
            /// @param method HTTP 请求方法
            /// @param url 请求的 URL
            /// @param headers 请求 Header
            /// @param body 请求 Body 的地址
            /// @param body_size 请求 Body 的长度，单位为字节
            /// @return Authorization 的值
            std::string authorization_v2_for_request(const std::string &method, const std::string &url, const std::map<std::string, std::string> &headers, const char *body, size_t body_size) const;

            /// @brief  使用七牛签名算法 V2 对 HTTP 请求（请求体为输入流）进行签名，返回 Authorization 的值
            /// @param method HTTP 请求方法
            /// @param url 请求的 URL
            /// @param headers 请求 Header
            /// @param body_reader 请求 Body 的输入流
            /// @return Authorization 的值
            std::string authorization_v2_for_request_with_body_reader(const std::string &method, const std::string &url, const std::map<std::string, std::string> &headers, std::istream *body_reader) const;

            /// @brief 对对象的下载 URL 签名，可以生成私有存储空间的下载地址
            /// @param url 需要签名的 URL
            /// @param lifetime 签名有效期
            /// @return 经过签名的 URL
            std::string sign_download_url(const std::string &url, const std::chrono::nanoseconds &lifetime) const;

        private:
            rust::Box<_internal::rust_sdk_ffi::Credential> inner;
            friend class CredentialProvider;
        };

        /// @brief 获取的认证信息
        /// @details 该数据结构目前和认证信息相同，可以转换为认证信息使用，但之后可能会添加更多字段
        class GotCredential final
        {
        public:
            /// @private
            GotCredential(rust::Box<_internal::rust_sdk_ffi::GotCredential> &&got_credential) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::GotCredential &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::GotCredential &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GotCredential> inner;
        };

        /// @brief 获取认证信息的选项
        class GetCredentialOptions final
        {
        public:
            /// @brief 构造获取认证信息的选项
            GetCredentialOptions() noexcept;
            /// @brief 复制获取构造认证信息的选项
            GetCredentialOptions(const GetCredentialOptions &) noexcept;
            /// @private
            GetCredentialOptions(rust::Box<_internal::rust_sdk_ffi::GetCredentialOptions> &&options) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::GetCredentialOptions &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::GetCredentialOptions &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GetCredentialOptions> inner;
        };

        /// @brief 认证信息获取类
        class CredentialProvider
        {
        public:
            CredentialProvider() = delete;
            /// @brief 复制构造认证信息获取类
            CredentialProvider(const CredentialProvider &) noexcept;
            /// @brief 将认证信息转换为认证信息获取类
            CredentialProvider(Credential &&credential) noexcept;
            /// @private
            CredentialProvider(rust::Box<_internal::rust_sdk_ffi::CredentialProvider> &&provider) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::CredentialProvider &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::CredentialProvider &operator*() noexcept;
            /// @brief 获取七牛认证信息
            /// @return 七牛认证信息
            virtual GotCredential get(const GetCredentialOptions &options = GetCredentialOptions()) const final;

        private:
            rust::Box<_internal::rust_sdk_ffi::CredentialProvider> inner;
            friend class ChainCredentialsProviderBuilder;
            friend class upload_token::UploadTokenProvider;
            friend class upload_token::BucketUploadTokenProvider;
            friend class upload_token::ObjectUploadTokenProvider;
        };

        /// @brief 全局认证信息提供者，可以将认证信息配置在全局变量中。任何全局认证信息提供者实例都可以设置和访问全局认证信息。
        class GlobalCredentialProvider final : public CredentialProvider
        {
        public:
            /// @brief 构造全局认证信息提供者
            GlobalCredentialProvider() noexcept;

            /// @brief 配置全局认证信息
            /// @param credential 配置的认证信息
            static void setup(const Credential &credential) noexcept;

            /// @brief 清空全局认证信息
            static void clear() noexcept;
        };

        /// @brief 环境变量认证信息提供者，可以将认证信息配置在环境变量中。
        class EnvCredentialProvider final : public CredentialProvider
        {
        public:
            /// @brief 构造环境变量认证信息提供者
            EnvCredentialProvider() noexcept;
            /// @brief 配置环境变量认证信息提供者
            /// @param credential 配置的认证信息
            static void setup(const Credential &credential) noexcept;
            /// @brief 清空环境变量认证信息
            static void clear() noexcept;
        };

        class ChainCredentialsProvider;

        /// @brief 串联认证信息构建器
        /// @details 接受多个认证信息获取接口的实例并将他们串联成串联认证信息
        class ChainCredentialsProviderBuilder final
        {
        public:
            ChainCredentialsProviderBuilder() = delete;
            /// @brief 构造串联认证信息构建器
            /// @param provider 第一个认证信息提供者
            ChainCredentialsProviderBuilder(CredentialProvider &&provider) noexcept;
            /// @private
            ChainCredentialsProviderBuilder(rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> &&provider) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() noexcept;
            /// @brief 将认证信息提供者推送到认证串末端
            /// @param provider 认证信息提供者
            void append_credential(CredentialProvider &&provider) noexcept;
            /// @brief 将认证信息提供者推送到认证串顶端
            /// @param provider 认证信息提供者
            void prepend_credential(CredentialProvider &&provider) noexcept;
            /// @brief 构造认证信息串提供者
            ChainCredentialsProvider build() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> inner;
        };

        /// @brief 认证信息串提供者
        /// @details 将多个认证信息提供者串联，遍历并找寻第一个可用认证信息
        class ChainCredentialsProvider final : public CredentialProvider
        {
        public:
            ChainCredentialsProvider() = delete;
            /// @brief 构造认证信息串提供者
            /// @param builder 串联认证信息构建器
            ChainCredentialsProvider(ChainCredentialsProviderBuilder &builder) noexcept;
        };
    }

    namespace upload_token
    {
        class UploadPolicy;
        class GotUploadPolicy;

        const uint8_t STANDARD = 1;
        const uint8_t INFREQUENT_ACCESS = 2;
        const uint8_t ARCHIVE = 3;
        const uint8_t DEEP_ARCHIVE = 4;

        class UploadPolicyBuilder final
        {
        public:
            UploadPolicyBuilder() = delete;
            UploadPolicyBuilder(const UploadPolicyBuilder &) noexcept;
            /// @private
            UploadPolicyBuilder(rust::Box<_internal::rust_sdk_ffi::UploadPolicyBuilder> &&builder) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::UploadPolicyBuilder &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::UploadPolicyBuilder &operator*() noexcept;
            static UploadPolicyBuilder new_for_bucket(const std::string &bucket, const std::chrono::nanoseconds &lifetime) noexcept;
            static UploadPolicyBuilder new_for_object(const std::string &bucket, const std::string &object, const std::chrono::nanoseconds &lifetime) noexcept;
            static UploadPolicyBuilder new_for_objects_with_prefix(const std::string &bucket, const std::string &prefix, const std::chrono::nanoseconds &lifetime) noexcept;
            UploadPolicyBuilder &token_lifetime(const std::chrono::nanoseconds &lifetime) noexcept;
            UploadPolicyBuilder &token_deadline(const std::chrono::nanoseconds &time_since_epoch) noexcept;
            UploadPolicyBuilder &insert_only() noexcept;
            UploadPolicyBuilder &enable_mime_detection() noexcept;
            UploadPolicyBuilder &disable_mime_detection() noexcept;
            UploadPolicyBuilder &file_type(uint8_t file_type) noexcept;
            UploadPolicyBuilder &return_url(const std::string &url) noexcept;
            UploadPolicyBuilder &return_body(const std::string &body) noexcept;
            UploadPolicyBuilder &callback(const std::string *urls, const size_t urls_count, const std::string &host = "", const std::string &body = "", const std::string &body_type = "") noexcept;
            UploadPolicyBuilder &save_as(const std::string &save_as, bool force = false) noexcept;
            UploadPolicyBuilder &file_size_limitation(uint64_t min_file_size = 0, uint64_t max_file_size = 0) noexcept;
            UploadPolicyBuilder &mime_types(const std::string *mime_types, const size_t mime_types_count) noexcept;
            UploadPolicyBuilder &object_lifetime(const std::chrono::nanoseconds &lifetime) noexcept;
            UploadPolicyBuilder &set_as_string(const std::string &key, const std::string &value) noexcept;
            UploadPolicyBuilder &set_as_integer(const std::string &key, int64_t value) noexcept;
            UploadPolicyBuilder &set_as_bool(const std::string &key, bool value) noexcept;
            UploadPolicyBuilder &unset(const std::string &key) noexcept;
            UploadPolicyBuilder &reset() noexcept;
            UploadPolicy build() const noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::UploadPolicyBuilder> inner;
        };

        class UploadPolicy final
        {
        public:
            UploadPolicy() = delete;
            UploadPolicy(const UploadPolicy &) noexcept;
            UploadPolicy(const GotUploadPolicy &) noexcept;
            UploadPolicy(const UploadPolicyBuilder &) noexcept;
            /// @private
            UploadPolicy(rust::Box<_internal::rust_sdk_ffi::UploadPolicy> &&builder) noexcept;
            static UploadPolicy from_json(const std::string &json);
            /// @private
            const _internal::rust_sdk_ffi::UploadPolicy &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::UploadPolicy &operator*() noexcept;
            std::string bucket() const noexcept;
            std::string key() const noexcept;
            bool has_prefixal_object_key() const noexcept;
            bool is_insert_only() const noexcept;
            bool is_mime_detection_enabled() const noexcept;
            std::chrono::nanoseconds token_deadline() const;
            std::string return_url() const noexcept;
            std::string return_body() const noexcept;
            std::vector<std::string> callback_urls() const noexcept;
            std::string callback_host() const noexcept;
            std::string callback_body() const noexcept;
            std::string callback_body_type() const noexcept;
            std::string save_key() const noexcept;
            bool is_save_key_forced() const noexcept;
            uint64_t max_file_size() const noexcept;
            uint64_t min_file_size() const noexcept;
            std::vector<std::string> mime_types() const noexcept;
            uint8_t file_type() const noexcept;
            std::chrono::nanoseconds object_lifetime() const noexcept;
            std::string to_json() const noexcept;
            bool has_key(const std::string &key) const noexcept;
            std::string get_as_string(const std::string &key) const noexcept;
            int64_t get_as_integer(const std::string &key) const noexcept;
            bool get_as_bool(const std::string &key) const noexcept;
            bool is_string(const std::string &key) const noexcept;
            bool is_integer(const std::string &key) const noexcept;
            bool is_bool(const std::string &key) const noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::UploadPolicy> inner;
            friend class UploadTokenProvider;
        };

        /// @brief 获取 Access Key 的选项
        class GetAccessKeyOptions final
        {
        public:
            /// @brief 构造获取 Access Key 的选项
            GetAccessKeyOptions() noexcept;
            /// @brief 复制构造获取 Access Key 的选项
            GetAccessKeyOptions(const GetAccessKeyOptions &) noexcept;
            /// @private
            GetAccessKeyOptions(rust::Box<_internal::rust_sdk_ffi::GetAccessKeyOptions> &&options) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::GetAccessKeyOptions &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::GetAccessKeyOptions &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GetAccessKeyOptions> inner;
        };

        /// @brief 获取上传策略的选项
        class GetPolicyOptions final
        {
        public:
            /// @brief 构造获取上传策略的选项
            GetPolicyOptions() noexcept;
            /// @brief 复制构造获取上传策略的选项
            GetPolicyOptions(const GetPolicyOptions &) noexcept;
            /// @private
            GetPolicyOptions(rust::Box<_internal::rust_sdk_ffi::GetPolicyOptions> &&options) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::GetPolicyOptions &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::GetPolicyOptions &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GetPolicyOptions> inner;
        };

        /// @brief 转换为上传凭证字符串的选项
        class ToUploadTokenStringOptions final
        {
        public:
            /// @brief 构造转换为上传凭证字符串的选项
            ToUploadTokenStringOptions() noexcept;
            /// @brief 复制构造转换为上传凭证字符串的选项
            ToUploadTokenStringOptions(const ToUploadTokenStringOptions &) noexcept;
            /// @private
            ToUploadTokenStringOptions(rust::Box<_internal::rust_sdk_ffi::ToUploadTokenStringOptions> &&options) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::ToUploadTokenStringOptions &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::ToUploadTokenStringOptions &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::ToUploadTokenStringOptions> inner;
        };

        /// @brief 获取的上传策略
        /// @details 该数据结构目前和上传策略相同，可以转换为上传策略使用，但之后可能会添加更多字段
        class GotUploadPolicy final
        {
        public:
            /// @private
            GotUploadPolicy(rust::Box<_internal::rust_sdk_ffi::GotUploadPolicy> &&got_upload_policy) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::GotUploadPolicy &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::GotUploadPolicy &operator*() noexcept;

        private:
            rust::Box<_internal::rust_sdk_ffi::GotUploadPolicy> inner;
        };

        /// @brief 上传凭证获取类
        class UploadTokenProvider
        {
        public:
            UploadTokenProvider() = delete;
            /// @brief 复制构造上传凭证获取类
            UploadTokenProvider(const UploadTokenProvider &) noexcept;
            /// @brief 将上传策略转换为上传凭证获取类
            UploadTokenProvider(UploadPolicy &&, credential::CredentialProvider &&) noexcept;
            /// @brief 将上传凭证字符串转换为上传凭证获取类
            UploadTokenProvider(const std::string &token) noexcept;
            /// @private
            UploadTokenProvider(rust::Box<_internal::rust_sdk_ffi::UploadTokenProvider> &&provider) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::UploadTokenProvider &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::UploadTokenProvider &operator*() noexcept;
            /// @brief 从上传凭证内获取 AccessKey
            /// @return AccessKey
            virtual std::string get_access_key(const GetAccessKeyOptions &options = GetAccessKeyOptions()) const final;
            /// @brief 从上传凭证内获取上传策略
            /// @return 上传策略
            virtual GotUploadPolicy get_upload_policy(const GetPolicyOptions &options = GetPolicyOptions()) const final;
            /// @brief 转换为上传凭证字符串
            /// @return 上传凭证字符串
            virtual std::string to_token_string(const ToUploadTokenStringOptions &options = ToUploadTokenStringOptions()) const final;
            /// @brief 从上传凭证内获取存储空间名称
            /// @return 存储空间名称
            virtual std::string get_bucket_name(const GetPolicyOptions &options = GetPolicyOptions()) const final;

        private:
            rust::Box<_internal::rust_sdk_ffi::UploadTokenProvider> inner;
        };

        /// @brief 基于存储空间的动态生成
        /// @details 根据存储空间的快速生成上传凭证实例
        class BucketUploadTokenProvider final : public UploadTokenProvider
        {
        public:
            BucketUploadTokenProvider() = delete;
            /// @brief 构造存储空间上传凭证
            /// @param bucket 存储空间名称
            /// @param lifetime 上传凭证有效期
            /// @param credential 认证信息
            BucketUploadTokenProvider(const std::string &bucket, const std::chrono::nanoseconds &lifetime, credential::CredentialProvider &&credential) noexcept;
        };

        /// @brief 基于对象的动态生成
        /// @details 根据对象的快速生成上传凭证实例
        class ObjectUploadTokenProvider final : public UploadTokenProvider
        {
        public:
            ObjectUploadTokenProvider() = delete;
            /// @brief 构造对象上传凭证
            /// @param bucket 存储空间名称
            /// @param object 对象名称
            /// @param lifetime 上传凭证有效期
            /// @param credential 认证信息
            ObjectUploadTokenProvider(const std::string &bucket, const std::string &object, const std::chrono::nanoseconds &lifetime, credential::CredentialProvider &&credential) noexcept;
        };
    }

    static int _ = _internal::initialize();
}

#endif // ___QINIU_SDK_DEFINED
