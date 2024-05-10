// https://github.com/nurmohammed840/websocket.rs/blob/main/examples/utils/handshake.rs

use tokio::io::AsyncBufReadExt;

pub struct HttpRequest {
    pub prefix: String,
    headers: std::collections::HashMap<String, String>,
}

pub trait Header {
    /// Format a single http header field
    fn fmt(_: &Self) -> String;
}

pub fn get_sec_key(req: &HttpRequest) -> Option<&String> {
    if !req
        .headers
        .get("connection")?
        .eq_ignore_ascii_case("upgrade")
        || !req
            .headers
            .get("upgrade")?
            .eq_ignore_ascii_case("websocket")
    {
        return None;
    }

    req.headers.get("sec-websocket-key")
}

impl HttpRequest {
    pub async fn parse<IO>(reader: &mut IO) -> std::io::Result<Self>
    where
        IO: Unpin + tokio::io::AsyncBufRead,
    {
        let mut lines = reader.lines();

        let prefix = lines.next_line().await?.unwrap();
        let mut headers = std::collections::HashMap::new();

        while let Some(line) = lines.next_line().await? {
            if line.is_empty() {
                break;
            }

            let (key, value) = line.split_once(":").unwrap();
            headers.insert(key.to_ascii_lowercase(), value.trim_start().into());
        }

        Ok(Self { prefix, headers })
    }
}

impl<T: Header> Header for &T {
    fn fmt(this: &Self) -> String {
        T::fmt(this)
    }
}
impl<T: std::fmt::Display> Header for [T; 2] {
    fn fmt([key, value]: &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}
impl<K: std::fmt::Display, V: std::fmt::Display> Header for (K, V) {
    fn fmt((key, value): &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}
