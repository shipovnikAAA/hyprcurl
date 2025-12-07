//! Type-safe wrappers for curl options and info

use curl_sys::*;

/// Curl option type-safe wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlOpt {
    Url,
    Port,
    Proxy,
    ProxyPort,
    ProxyUserPwd,
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
    SslCipherList,
    SslCurves,
    AcceptEncoding,
}

impl CurlOpt {
    /// Convert to raw curl option code
    pub fn to_raw(self) -> CURLoption {
        match self {
            CurlOpt::Url => CURLOPT_URL,
            CurlOpt::Port => CURLOPT_PORT,
            CurlOpt::Proxy => CURLOPT_PROXY,
            CurlOpt::ProxyPort => CURLOPT_PROXYPORT,
            CurlOpt::ProxyUserPwd => CURLOPT_PROXYUSERPWD,
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
            CurlOpt::SslCipherList => CURLOPT_SSL_CIPHER_LIST,
            CurlOpt::SslCurves => CURLOPT_SSLVERSION, // Use CURLOPT_SSLVERSION as placeholder
            CurlOpt::AcceptEncoding => CURLOPT_ACCEPT_ENCODING,
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

/// Browser impersonation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Browser {
    /// Google Chrome browser with specific version
    Chrome { version: u32 },
    /// Google Chrome browser (auto-latest version)
    ChromeLatest,
    /// Mozilla Firefox browser with specific version
    Firefox { version: u32 },
    /// Mozilla Firefox browser (auto-latest version)
    FirefoxLatest,
    /// Apple Safari browser with specific version
    Safari { version: String },
    /// Apple Safari browser (auto-latest version)
    SafariLatest,
    /// Microsoft Edge browser with specific version
    Edge { version: u32 },
    /// Microsoft Edge browser (auto-latest version)
    EdgeLatest,
    /// Tor browser with specific version
    Tor { version: String },
    /// Tor browser (auto-latest version)
    TorLatest,
}

impl Browser {
    // Default latest versions
    const CHROME_LATEST: u32 = 131;
    const FIREFOX_LATEST: u32 = 121;
    const SAFARI_LATEST: &'static str = "18.0";
    const EDGE_LATEST: u32 = 131;
    const TOR_LATEST: &'static str = "13.5";

    /// Get default user agent string for the browser
    pub fn user_agent(&self) -> String {
        match self {
            Browser::Chrome { version } => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
            Browser::ChromeLatest => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    Self::CHROME_LATEST
                )
            }
            Browser::Firefox { version } => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{}) Gecko/20100101 Firefox/{}.0",
                    version, version
                )
            }
            Browser::FirefoxLatest => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{}) Gecko/20100101 Firefox/{}.0",
                    Self::FIREFOX_LATEST,
                    Self::FIREFOX_LATEST
                )
            }
            Browser::Safari { version } => {
                format!(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{} Safari/605.1.15",
                    version
                )
            }
            Browser::SafariLatest => {
                format!(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{} Safari/605.1.15",
                    Self::SAFARI_LATEST
                )
            }
            Browser::Edge { version } => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36 Edg/{}.0.0.0",
                    version, version
                )
            }
            Browser::EdgeLatest => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36 Edg/{}.0.0.0",
                    Self::EDGE_LATEST, Self::EDGE_LATEST
                )
            }
            Browser::Tor { version } => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/{}.0",
                    version
                )
            }
            Browser::TorLatest => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/{}.0",
                    Self::TOR_LATEST
                )
            }
        }
    }

    /// Get browser-specific HTTP headers
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        match self {
            Browser::Chrome { .. } | Browser::ChromeLatest => {
                let version = match self {
                    Browser::Chrome { version } => *version,
                    _ => Self::CHROME_LATEST,
                };

                vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string()),
                    ("Accept-Language", "en-US,en;q=0.9".to_string()),
                    ("Accept-Encoding", "gzip, deflate, br".to_string()),
                    ("sec-ch-ua", format!("\"Google Chrome\";v=\"{}\", \"Chromium\";v=\"{}\", \"Not A(Brand\";v=\"24\"", version, version)),
                    ("sec-ch-ua-mobile", "?0".to_string()),
                    ("sec-ch-ua-platform", "\"Windows\"".to_string()),
                    ("Sec-Fetch-Dest", "document".to_string()),
                    ("Sec-Fetch-Mode", "navigate".to_string()),
                    ("Sec-Fetch-Site", "none".to_string()),
                    ("Sec-Fetch-User", "?1".to_string()),
                    ("Upgrade-Insecure-Requests", "1".to_string()),
                ]
            }
            Browser::Firefox { .. } | Browser::FirefoxLatest => {
                vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string()),
                    ("Accept-Language", "en-US,en;q=0.5".to_string()),
                    ("Accept-Encoding", "gzip, deflate, br".to_string()),
                    ("DNT", "1".to_string()),
                    ("Connection", "keep-alive".to_string()),
                    ("Upgrade-Insecure-Requests", "1".to_string()),
                    ("Sec-Fetch-Dest", "document".to_string()),
                    ("Sec-Fetch-Mode", "navigate".to_string()),
                    ("Sec-Fetch-Site", "none".to_string()),
                    ("Sec-Fetch-User", "?1".to_string()),
                ]
            }
            Browser::Safari { .. } | Browser::SafariLatest => {
                vec![
                    (
                        "Accept",
                        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
                            .to_string(),
                    ),
                    ("Accept-Language", "en-US,en;q=0.9".to_string()),
                    ("Accept-Encoding", "gzip, deflate, br".to_string()),
                    ("Upgrade-Insecure-Requests", "1".to_string()),
                ]
            }
            Browser::Edge { .. } | Browser::EdgeLatest => {
                let version = match self {
                    Browser::Edge { version } => *version,
                    _ => Self::EDGE_LATEST,
                };

                vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".to_string()),
                    ("Accept-Language", "en-US,en;q=0.9".to_string()),
                    ("Accept-Encoding", "gzip, deflate, br".to_string()),
                    ("sec-ch-ua", format!("\"Microsoft Edge\";v=\"{}\", \"Chromium\";v=\"{}\", \"Not A(Brand\";v=\"24\"", version, version)),
                    ("sec-ch-ua-mobile", "?0".to_string()),
                    ("sec-ch-ua-platform", "\"Windows\"".to_string()),
                    ("Sec-Fetch-Dest", "document".to_string()),
                    ("Sec-Fetch-Mode", "navigate".to_string()),
                    ("Sec-Fetch-Site", "none".to_string()),
                    ("Sec-Fetch-User", "?1".to_string()),
                    ("Upgrade-Insecure-Requests", "1".to_string()),
                ]
            }
            Browser::Tor { .. } | Browser::TorLatest => {
                vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".to_string()),
                    ("Accept-Language", "en-US,en;q=0.5".to_string()),
                    ("Accept-Encoding", "gzip, deflate".to_string()),
                    ("DNT", "1".to_string()),
                    ("Connection", "keep-alive".to_string()),
                    ("Upgrade-Insecure-Requests", "1".to_string()),
                ]
            }
        }
    }

    /// Get TLS cipher suites for the browser
    pub fn tls_ciphers(&self) -> &'static str {
        match self {
            Browser::Chrome { .. } | Browser::ChromeLatest | Browser::Edge { .. } | Browser::EdgeLatest => {
                "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384"
            }
            Browser::Firefox { .. } | Browser::FirefoxLatest | Browser::Tor { .. } | Browser::TorLatest => {
                "TLS_AES_128_GCM_SHA256:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_256_GCM_SHA384:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES256-GCM-SHA384"
            }
            Browser::Safari { .. } | Browser::SafariLatest => {
                "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-CHACHA20-POLY1305"
            }
        }
    }

    /// Get TLS curves for the browser
    pub fn tls_curves(&self) -> &'static str {
        match self {
            Browser::Chrome { .. }
            | Browser::ChromeLatest
            | Browser::Edge { .. }
            | Browser::EdgeLatest => "X25519:P-256:P-384:P-521:X25519Kyber768Draft00",
            Browser::Firefox { .. }
            | Browser::FirefoxLatest
            | Browser::Tor { .. }
            | Browser::TorLatest => "X25519:P-256:P-384:P-521",
            Browser::Safari { .. } | Browser::SafariLatest => "X25519:P-256:P-384:P-521",
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
