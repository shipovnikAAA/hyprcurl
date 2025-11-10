//! Type-safe wrappers for curl options and info

use curl_sys::*;

/// Curl option type-safe wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlOpt {
    Url,
    Port,
    Proxy,
    ProxyPort,
    Timeout,
    ConnectTimeout,
    FollowLocation,
    MaxRedirs,
    UserAgent,
    HttpHeader,
    PostFields,
    Verbose,
    NoSignal,
    SslVerifyPeer,
    SslVerifyHost,
    CaInfo,
    CaPath,
    Cookie,
    CookieFile,
    CookieJar,
    HttpVersion,
    CustomRequest,
    SslCert,
    SslKey,
    SslKeyType,
    SslEngine,
    SslDefaultEngine,
    ProxyCaInfo,
    ProxyCaPath,
    ProxySslCert,
    ProxySslKey,
    ProxySslVerifyPeer,
    ProxySslVerifyHost,
}

impl CurlOpt {
    /// Convert to raw curl option code
    pub fn to_raw(self) -> CURLoption {
        match self {
            CurlOpt::Url => CURLOPT_URL,
            CurlOpt::Port => CURLOPT_PORT,
            CurlOpt::Proxy => CURLOPT_PROXY,
            CurlOpt::ProxyPort => CURLOPT_PROXYPORT,
            CurlOpt::Timeout => CURLOPT_TIMEOUT,
            CurlOpt::ConnectTimeout => CURLOPT_CONNECTTIMEOUT,
            CurlOpt::FollowLocation => CURLOPT_FOLLOWLOCATION,
            CurlOpt::MaxRedirs => CURLOPT_MAXREDIRS,
            CurlOpt::UserAgent => CURLOPT_USERAGENT,
            CurlOpt::HttpHeader => CURLOPT_HTTPHEADER,
            CurlOpt::PostFields => CURLOPT_POSTFIELDS,
            CurlOpt::Verbose => CURLOPT_VERBOSE,
            CurlOpt::NoSignal => CURLOPT_NOSIGNAL,
            CurlOpt::SslVerifyPeer => CURLOPT_SSL_VERIFYPEER,
            CurlOpt::SslVerifyHost => CURLOPT_SSL_VERIFYHOST,
            CurlOpt::CaInfo => CURLOPT_CAINFO,
            CurlOpt::CaPath => CURLOPT_CAPATH,
            CurlOpt::Cookie => CURLOPT_COOKIE,
            CurlOpt::CookieFile => CURLOPT_COOKIEFILE,
            CurlOpt::CookieJar => CURLOPT_COOKIEJAR,
            CurlOpt::HttpVersion => CURLOPT_HTTP_VERSION,
            CurlOpt::CustomRequest => CURLOPT_CUSTOMREQUEST,
            CurlOpt::SslCert => CURLOPT_SSLCERT,
            CurlOpt::SslKey => CURLOPT_SSLKEY,
            CurlOpt::SslKeyType => CURLOPT_SSLKEYTYPE,
            CurlOpt::SslEngine => CURLOPT_SSLENGINE,
            CurlOpt::SslDefaultEngine => CURLOPT_SSLENGINE_DEFAULT,
            CurlOpt::ProxyCaInfo => CURLOPT_PROXY_CAINFO,
            CurlOpt::ProxyCaPath => CURLOPT_PROXY_CAPATH,
            CurlOpt::ProxySslCert => CURLOPT_PROXY_SSLCERT,
            CurlOpt::ProxySslKey => CURLOPT_PROXY_SSLKEY,
            CurlOpt::ProxySslVerifyPeer => CURLOPT_PROXY_SSL_VERIFYPEER,
            CurlOpt::ProxySslVerifyHost => CURLOPT_PROXY_SSL_VERIFYHOST,
        }
    }
}

/// Curl info type-safe wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlInfo {
    ResponseCode,
    TotalTime,
    NameLookupTime,
    ConnectTime,
    PreTransferTime,
    StartTransferTime,
    RedirectTime,
    RedirectCount,
    SizeDownload,
    SizeUpload,
    SpeedDownload,
    SpeedUpload,
    ContentType,
    EffectiveUrl,
}

impl CurlInfo {
    /// Convert to raw curl info code
    pub fn to_raw(self) -> CURLINFO {
        match self {
            CurlInfo::ResponseCode => CURLINFO_RESPONSE_CODE,
            CurlInfo::TotalTime => CURLINFO_TOTAL_TIME,
            CurlInfo::NameLookupTime => CURLINFO_NAMELOOKUP_TIME,
            CurlInfo::ConnectTime => CURLINFO_CONNECT_TIME,
            CurlInfo::PreTransferTime => CURLINFO_PRETRANSFER_TIME,
            CurlInfo::StartTransferTime => CURLINFO_STARTTRANSFER_TIME,
            CurlInfo::RedirectTime => CURLINFO_REDIRECT_TIME,
            CurlInfo::RedirectCount => CURLINFO_REDIRECT_COUNT,
            CurlInfo::SizeDownload => CURLINFO_SIZE_DOWNLOAD,
            CurlInfo::SizeUpload => CURLINFO_SIZE_UPLOAD,
            CurlInfo::SpeedDownload => CURLINFO_SPEED_DOWNLOAD,
            CurlInfo::SpeedUpload => CURLINFO_SPEED_UPLOAD,
            CurlInfo::ContentType => CURLINFO_CONTENT_TYPE,
            CurlInfo::EffectiveUrl => CURLINFO_EFFECTIVE_URL,
        }
    }
}

/// HTTP version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    /// HTTP/1.0
    V1_0,
    /// HTTP/1.1
    V1_1,
    /// HTTP/2
    V2,
    /// HTTP/2 TLS only
    V2Tls,
    /// HTTP/2 prior knowledge
    V2PriorKnowledge,
    /// HTTP/3
    V3,
}

impl HttpVersion {
    /// Convert to curl constant
    pub fn to_curl(self) -> i64 {
        match self {
            HttpVersion::V1_0 => CURL_HTTP_VERSION_1_0 as i64,
            HttpVersion::V1_1 => CURL_HTTP_VERSION_1_1 as i64,
            HttpVersion::V2 => CURL_HTTP_VERSION_2_0 as i64,
            HttpVersion::V2Tls => CURL_HTTP_VERSION_2TLS as i64,
            HttpVersion::V2PriorKnowledge => CURL_HTTP_VERSION_2_PRIOR_KNOWLEDGE as i64,
            HttpVersion::V3 => CURL_HTTP_VERSION_3 as i64,
        }
    }
}

/// WebSocket frame flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WsFlags(pub u32);

impl WsFlags {
    pub const TEXT: WsFlags = WsFlags(1 << 0);
    pub const BINARY: WsFlags = WsFlags(1 << 1);
    pub const CONT: WsFlags = WsFlags(1 << 2);
    pub const CLOSE: WsFlags = WsFlags(1 << 3);
    pub const PING: WsFlags = WsFlags(1 << 4);
    pub const PONG: WsFlags = WsFlags(1 << 5);
}
