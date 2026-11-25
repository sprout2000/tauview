use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::SystemTime;
use tauri::http::{header::CONTENT_TYPE, Request, Response, StatusCode};

use crate::format::{self, Format};

/// 変換結果をメモリに保持する数（サムネイルは小さいので多めに、等倍は大きいので少なめに）
const THUMB_CACHE_SIZE: usize = 512;
const FULL_CACHE_SIZE: usize = 8;

/// ファイルが変わったら別のキーになるように、更新日時とサイズも含める
#[derive(Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    path: PathBuf,
    modified: Option<SystemTime>,
    len: u64,
}

type Converted = Arc<(Vec<u8>, &'static str)>;

/// 古いものから捨てる簡単なキャッシュ
struct Cache {
    capacity: usize,
    map: HashMap<CacheKey, Converted>,
    order: VecDeque<CacheKey>,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&self, key: &CacheKey) -> Option<Converted> {
        self.map.get(key).cloned()
    }

    fn insert(&mut self, key: CacheKey, value: Converted) {
        if self.map.insert(key.clone(), value).is_none() {
            self.order.push_back(key);
        }
        while self.order.len() > self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
    }
}

static THUMB_CACHE: LazyLock<Mutex<Cache>> =
    LazyLock::new(|| Mutex::new(Cache::new(THUMB_CACHE_SIZE)));
static FULL_CACHE: LazyLock<Mutex<Cache>> =
    LazyLock::new(|| Mutex::new(Cache::new(FULL_CACHE_SIZE)));

fn cache_key(path: &Path) -> Option<CacheKey> {
    let meta = std::fs::metadata(path).ok()?;
    Some(CacheKey {
        path: path.to_path_buf(),
        modified: meta.modified().ok(),
        len: meta.len(),
    })
}

/// 変換が必要な画像を、キャッシュがあればそれを、なければ変換して返す
fn convert_cached(path: &Path, format: Format, thumbnail: bool) -> Result<Converted, String> {
    let cache = if thumbnail { &THUMB_CACHE } else { &FULL_CACHE };
    let key = cache_key(path).ok_or("failed to read metadata")?;

    if let Some(hit) = cache.lock().unwrap().get(&key) {
        return Ok(hit);
    }

    // 変換は時間がかかるので、ロックを外して行う
    let converted = Arc::new(format::convert(path, format, thumbnail)?);
    cache.lock().unwrap().insert(key, converted.clone());
    Ok(converted)
}

/// URL のクエリに `size=thumb` があればサムネイルを要求している
fn wants_thumbnail(query: Option<&str>) -> bool {
    query
        .map(|q| q.split('&').any(|pair| pair == "size=thumb"))
        .unwrap_or(false)
}

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

/// 画像を WebView に渡す。WebView が表示できない形式（RAW・ICNS）は変換する
pub fn handle(request: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let Some(path) = decode_path(request.uri().path().trim_start_matches('/')) else {
        return error(StatusCode::BAD_REQUEST);
    };

    let Some(format) = format::detect(&path) else {
        return error(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    };

    let result = if format.is_native() {
        std::fs::read(&path)
            .map(|bytes| (bytes, format.mime()))
            .map_err(|e| e.to_string())
    } else {
        let thumbnail = wants_thumbnail(request.uri().query());
        convert_cached(&path, format, thumbnail).map(|c| (c.0.clone(), c.1))
    };

    match result {
        Ok((bytes, mime)) => Response::builder()
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

    #[test]
    fn parses_thumbnail_query() {
        assert!(wants_thumbnail(Some("size=thumb")));
        assert!(wants_thumbnail(Some("a=1&size=thumb")));
        assert!(!wants_thumbnail(Some("size=full")));
        assert!(!wants_thumbnail(None));
    }
}
