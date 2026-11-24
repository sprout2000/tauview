use std::path::PathBuf;
use tauri::http::{header::CONTENT_TYPE, Request, Response, StatusCode};

use crate::format;

/// `convertFileSrc(path, "tauview")` で作った URL のパス部分を元のファイルパスに戻す
fn decode_path(encoded: &str) -> Option<PathBuf> {
    let bytes = encoded.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok().map(PathBuf::from)
}

fn error(status: StatusCode) -> Response<Vec<u8>> {
    Response::builder().status(status).body(Vec::new()).unwrap()
}

/// 画像を WebView に渡す。WebView が表示できない形式は PNG に変換する
pub fn handle(request: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let Some(path) = decode_path(request.uri().path().trim_start_matches('/')) else {
        return error(StatusCode::BAD_REQUEST);
    };

    let Some(format) = format::detect(&path) else {
        return error(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    };

    let (body, mime) = if format.is_native() {
        (
            std::fs::read(&path).map_err(|e| e.to_string()),
            format.mime(),
        )
    } else {
        (format::convert_to_png(&path), "image/png")
    };

    match body {
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime)
            .body(bytes)
            .unwrap(),
        Err(why) => {
            println!("Error while loading {}: {}", path.display(), why);
            error(StatusCode::NOT_FOUND)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_encoded_paths() {
        assert_eq!(
            decode_path("%2FUsers%2Ffoo%2F%E7%94%BB%E5%83%8F.png"),
            Some(PathBuf::from("/Users/foo/画像.png"))
        );
        assert_eq!(
            decode_path("C%3A%5CUsers%5Cfoo.png"),
            Some(PathBuf::from("C:\\Users\\foo.png"))
        );
        assert_eq!(decode_path("%2"), None);
    }
}
