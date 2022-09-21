#include <string>
#include <map>
#include <chrono>
#include "rust/cxx.h"
#include "qiniu-sdk-ffi/src/lib.rs.h"

#ifndef ___QINIU_SDK_DEFINED
#define ___QINIU_SDK_DEFINED

/// @brief  七牛 SDK 名字空间
namespace qiniu_sdk
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

    /// @brief  七牛认证信息名字空间
    namespace credential
    {
        class GotCredential;
        class CredentialProvider;
        class GlobalCredentialProvider;

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
            /// @brief 构造认证信息的选项
            GetCredentialOptions() noexcept;
            /// @brief 复制构造认证信息的选项
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
            CredentialProvider(Credential credential) noexcept;
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
            ChainCredentialsProviderBuilder(CredentialProvider provider) noexcept;
            /// @private
            ChainCredentialsProviderBuilder(rust::Box<_internal::rust_sdk_ffi::ChainCredentialsProviderBuilder> &&provider) noexcept;
            /// @private
            const _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() const noexcept;
            /// @private
            _internal::rust_sdk_ffi::ChainCredentialsProviderBuilder &operator*() noexcept;
            /// @brief 将认证信息提供者推送到认证串末端
            /// @param provider 认证信息提供者
            void append_credential(CredentialProvider provider) noexcept;
            /// @brief 将认证信息提供者推送到认证串顶端
            /// @param provider 认证信息提供者
            void prepend_credential(CredentialProvider provider) noexcept;
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

    static int _ = _internal::initialize();
}

#endif // ___QINIU_SDK_DEFINED
