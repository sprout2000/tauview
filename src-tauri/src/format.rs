use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Tauview が扱う画像形式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Jpeg,
    Png,
    Gif,
    Webp,
    Bmp,
    Ico,
    Svg,
    Heif,
    Tiff,
    Icns,
    Jxl,
    /// カメラの RAW（中身が TIFF と同じ構造の形式が多いので、拡張子で判定する）
    Raw,
}

/// カメラの RAW の拡張子
pub const RAW_EXTENSIONS: &[&str] = &[
    "3fr", "arw", "cr2", "cr3", "crw", "dcr", "dng", "erf", "iiq", "k25", "kdc", "mef", "mos",
    "mrw", "nef", "nrw", "orf", "pef", "raf", "raw", "rw2", "rwl", "sr2", "srf", "srw", "x3f",
];

/// ファイルダイアログで選べる拡張子
#[cfg(target_os = "macos")]
pub const EXTENSIONS: &[&str] = &[
    "bmp", "gif", "ico", "jpeg", "jpg", "png", "svg", "webp", "heic", "heif", "icns", "jxl", "tif",
    "tiff", "3fr", "arw", "cr2", "cr3", "crw", "dcr", "dng", "erf", "iiq", "k25", "kdc", "mef",
    "mos", "mrw", "nef", "nrw", "orf", "pef", "raf", "raw", "rw2", "rwl", "sr2", "srf", "srw",
    "x3f",
];

#[cfg(not(target_os = "macos"))]
pub const EXTENSIONS: &[&str] = &["bmp", "gif", "ico", "jpeg", "jpg", "png", "svg", "webp"];

/// グリッド表示のサムネイルの最大サイズ（px）。表示は 128px なので Retina 向けに 2 倍にする
pub const THUMB_PIXEL_SIZE: u32 = 256;

/// 判定のために読み込む先頭のバイト数（SVG の判定に余裕を持たせる）
const HEAD_SIZE: usize = 4096;

impl Format {
    pub fn mime(self) -> &'static str {
        match self {
            Format::Jpeg => "image/jpeg",
            Format::Png => "image/png",
            Format::Gif => "image/gif",
            Format::Webp => "image/webp",
            Format::Bmp => "image/bmp",
            Format::Ico => "image/vnd.microsoft.icon",
            Format::Svg => "image/svg+xml",
            Format::Heif => "image/heif",
            Format::Tiff => "image/tiff",
            Format::Icns => "image/x-icns",
            Format::Jxl => "image/jxl",
            Format::Raw => "application/octet-stream",
        }
    }

    /// WebView がそのまま表示できる形式か（false なら変換して渡す）
    pub fn is_native(self) -> bool {
        !matches!(self, Format::Icns | Format::Raw)
    }

    /// この OS で扱う形式か（HEIF・TIFF・ICNS・JPEG XL・RAW は macOS のみ）
    fn is_supported(self) -> bool {
        match self {
            Format::Heif | Format::Tiff | Format::Icns | Format::Jxl | Format::Raw => {
                cfg!(target_os = "macos")
            }
            _ => true,
        }
    }
}

fn read_head(path: &Path, buf: &mut [u8]) -> Option<usize> {
    let mut file = File::open(path).ok()?;
    let mut n = 0;
    while n < buf.len() {
        match file.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(m) => n += m,
            Err(_) => return None,
        }
    }
    Some(n)
}

fn is_heif(head: &[u8]) -> bool {
    const BRANDS: [&[u8; 4]; 10] = [
        b"heic", b"heix", b"heim", b"heis", b"hevc", b"hevx", b"hevm", b"hevs", b"mif1", b"msf1",
    ];
    head.len() >= 12 && &head[4..8] == b"ftyp" && BRANDS.iter().any(|b| &head[8..12] == *b)
}

fn is_tiff(head: &[u8]) -> bool {
    head.starts_with(b"II*\0") || head.starts_with(b"MM\0*")
}

fn is_icns(head: &[u8]) -> bool {
    head.starts_with(b"icns")
}

fn is_jxl(head: &[u8]) -> bool {
    // 裸のコードストリーム、または ISO BMFF コンテナ
    head.starts_with(&[0xFF, 0x0A])
        || head.starts_with(&[
            0x00, 0x00, 0x00, 0x0C, b'J', b'X', b'L', b' ', 0x0D, 0x0A, 0x87, 0x0A,
        ])
}

fn is_svg(head: &[u8]) -> bool {
    let text = String::from_utf8_lossy(head);
    let text = text.trim_start_matches('\u{feff}').trim_start();
    text.starts_with('<') && text.to_ascii_lowercase().contains("<svg")
}

fn sniff(head: &[u8]) -> Option<Format> {
    if let Some(kind) = infer::get(head) {
        let format = match kind.mime_type() {
            "image/jpeg" => Some(Format::Jpeg),
            "image/png" => Some(Format::Png),
            "image/gif" => Some(Format::Gif),
            "image/webp" => Some(Format::Webp),
            "image/bmp" => Some(Format::Bmp),
            "image/vnd.microsoft.icon" => Some(Format::Ico),
            _ => None,
        };
        if format.is_some() {
            return format;
        }
    }

    if is_heif(head) {
        Some(Format::Heif)
    } else if is_tiff(head) {
        Some(Format::Tiff)
    } else if is_icns(head) {
        Some(Format::Icns)
    } else if is_jxl(head) {
        Some(Format::Jxl)
    } else if is_svg(head) {
        Some(Format::Svg)
    } else {
        None
    }
}

fn is_raw_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| RAW_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

/// ファイルの中身から画像形式を判定する（この OS で扱わない形式は None）
pub fn detect<P: AsRef<Path>>(path: P) -> Option<Format> {
    let path = path.as_ref();
    let mut buf = [0u8; HEAD_SIZE];
    let n = read_head(path, &mut buf)?;

    let format = if is_raw_extension(path) {
        (n > 0).then_some(Format::Raw)
    } else {
        sniff(&buf[..n])
    };
    format.filter(|f| f.is_supported())
}

/// WebView が表示できない画像を変換し、(データ, MIME タイプ) を返す。
/// thumbnail が true なら、グリッド表示用に縮小する
#[cfg(target_os = "macos")]
pub fn convert(
    path: &Path,
    format: Format,
    thumbnail: bool,
) -> Result<(Vec<u8>, &'static str), String> {
    use crate::imageio::{self, Output};

    match format {
        Format::Raw => {
            let max_pixel = thumbnail.then_some(THUMB_PIXEL_SIZE);
            imageio::convert_photo(path, max_pixel).map(|data| (data, Output::Jpeg.mime()))
        }
        Format::Icns => imageio::convert_icon(path).map(|data| (data, Output::Png.mime())),
        _ => Err(format!("{format:?} does not need conversion")),
    }
}

#[cfg(not(target_os = "macos"))]
pub fn convert(
    _path: &Path,
    _format: Format,
    _thumbnail: bool,
) -> Result<(Vec<u8>, &'static str), String> {
    Err("conversion is not supported on this platform".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_new_formats() {
        assert_eq!(sniff(b"\0\0\0\x18ftypheic\0\0\0\0"), Some(Format::Heif));
        assert_eq!(sniff(b"\0\0\0\x18ftypavif\0\0\0\0"), None);
        assert_eq!(sniff(b"II*\0\x08\0\0\0"), Some(Format::Tiff));
        assert_eq!(sniff(b"MM\0*\0\0\0\x08"), Some(Format::Tiff));
        assert_eq!(sniff(b"icns\0\0\x10\0"), Some(Format::Icns));
        assert_eq!(sniff(&[0xFF, 0x0A, 0x00]), Some(Format::Jxl));
        assert_eq!(
            sniff(
                b"\xef\xbb\xbf<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"/>"
            ),
            Some(Format::Svg)
        );
        assert_eq!(sniff(b"<html><body></body></html>"), None);
    }

    #[test]
    fn detects_raw_by_extension() {
        assert!(is_raw_extension(Path::new("/photos/IMG_0001.CR2")));
        assert!(is_raw_extension(Path::new("DSC0001.nef")));
        assert!(!is_raw_extension(Path::new("photo.tiff")));
        assert!(!is_raw_extension(Path::new("raw")));
    }
}
