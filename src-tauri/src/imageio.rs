//! macOS の ImageIO で、WebView が表示できない画像（RAW・ICNS）を JPEG / PNG に変換する。
//!
//! ImageIO と CoreFoundation は安定した C API なので、追加のクレートを使わずに直接宣言して呼ぶ。

#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_void, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

type CFTypeRef = *const c_void;
type CFIndex = isize;

#[repr(C)]
struct Opaque {
    _data: [u8; 0],
}

const kCFNumberSInt64Type: CFIndex = 4;
const kCFNumberFloat64Type: CFIndex = 6;
const kCFStringEncodingUTF8: u32 = 0x0800_0100;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFBooleanTrue: CFTypeRef;
    static kCFTypeDictionaryKeyCallBacks: Opaque;
    static kCFTypeDictionaryValueCallBacks: Opaque;

    fn CFRelease(cf: CFTypeRef);
    fn CFURLCreateFromFileSystemRepresentation(
        allocator: CFTypeRef,
        buffer: *const u8,
        buf_len: CFIndex,
        is_directory: u8,
    ) -> CFTypeRef;
    fn CFDictionaryCreate(
        allocator: CFTypeRef,
        keys: *const CFTypeRef,
        values: *const CFTypeRef,
        num_values: CFIndex,
        key_call_backs: *const Opaque,
        value_call_backs: *const Opaque,
    ) -> CFTypeRef;
    fn CFNumberCreate(
        allocator: CFTypeRef,
        the_type: CFIndex,
        value_ptr: *const c_void,
    ) -> CFTypeRef;
    fn CFStringCreateWithCString(
        allocator: CFTypeRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFTypeRef;
    fn CFDataCreateMutable(allocator: CFTypeRef, capacity: CFIndex) -> CFTypeRef;
    fn CFDataGetLength(data: CFTypeRef) -> CFIndex;
    fn CFDataGetBytePtr(data: CFTypeRef) -> *const u8;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGImageGetWidth(image: CFTypeRef) -> usize;
    fn CGImageGetHeight(image: CFTypeRef) -> usize;
}

#[link(name = "ImageIO", kind = "framework")]
extern "C" {
    static kCGImageSourceCreateThumbnailFromImageAlways: CFTypeRef;
    static kCGImageSourceCreateThumbnailFromImageIfAbsent: CFTypeRef;
    static kCGImageSourceCreateThumbnailWithTransform: CFTypeRef;
    static kCGImageSourceThumbnailMaxPixelSize: CFTypeRef;
    static kCGImageDestinationLossyCompressionQuality: CFTypeRef;

    fn CGImageSourceCreateWithURL(url: CFTypeRef, options: CFTypeRef) -> CFTypeRef;
    fn CGImageSourceGetCount(source: CFTypeRef) -> usize;
    fn CGImageSourceGetPrimaryImageIndex(source: CFTypeRef) -> usize;
    fn CGImageSourceCreateImageAtIndex(
        source: CFTypeRef,
        index: usize,
        options: CFTypeRef,
    ) -> CFTypeRef;
    fn CGImageSourceCreateThumbnailAtIndex(
        source: CFTypeRef,
        index: usize,
        options: CFTypeRef,
    ) -> CFTypeRef;
    fn CGImageDestinationCreateWithData(
        data: CFTypeRef,
        type_: CFTypeRef,
        count: usize,
        options: CFTypeRef,
    ) -> CFTypeRef;
    fn CGImageDestinationAddImage(dest: CFTypeRef, image: CFTypeRef, properties: CFTypeRef);
    fn CGImageDestinationFinalize(dest: CFTypeRef) -> bool;
}

/// Create 系の関数で得た CoreFoundation オブジェクト。破棄時に CFRelease する
struct Cf(CFTypeRef);

impl Cf {
    fn new(ptr: CFTypeRef) -> Option<Self> {
        (!ptr.is_null()).then_some(Cf(ptr))
    }
}

impl Drop for Cf {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

fn dictionary(pairs: &[(CFTypeRef, CFTypeRef)]) -> Option<Cf> {
    let keys: Vec<CFTypeRef> = pairs.iter().map(|(k, _)| *k).collect();
    let values: Vec<CFTypeRef> = pairs.iter().map(|(_, v)| *v).collect();
    Cf::new(unsafe {
        CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            pairs.len() as CFIndex,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        )
    })
}

fn number_i64(value: i64) -> Option<Cf> {
    Cf::new(unsafe {
        CFNumberCreate(
            std::ptr::null(),
            kCFNumberSInt64Type,
            &value as *const i64 as *const c_void,
        )
    })
}

fn number_f64(value: f64) -> Option<Cf> {
    Cf::new(unsafe {
        CFNumberCreate(
            std::ptr::null(),
            kCFNumberFloat64Type,
            &value as *const f64 as *const c_void,
        )
    })
}

fn string(value: &CStr) -> Option<Cf> {
    Cf::new(unsafe {
        CFStringCreateWithCString(std::ptr::null(), value.as_ptr(), kCFStringEncodingUTF8)
    })
}

/// 変換後の形式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    /// 写真向け（RAW）
    Jpeg,
    /// 透過を保つ（ICNS）
    Png,
}

impl Output {
    pub fn mime(self) -> &'static str {
        match self {
            Output::Jpeg => "image/jpeg",
            Output::Png => "image/png",
        }
    }
}

fn open_source(path: &Path) -> Result<Cf, String> {
    let bytes = path.as_os_str().as_bytes();
    let url = Cf::new(unsafe {
        CFURLCreateFromFileSystemRepresentation(
            std::ptr::null(),
            bytes.as_ptr(),
            bytes.len() as CFIndex,
            0,
        )
    })
    .ok_or("CFURLCreateFromFileSystemRepresentation failed")?;

    Cf::new(unsafe { CGImageSourceCreateWithURL(url.0, std::ptr::null()) })
        .ok_or_else(|| "CGImageSourceCreateWithURL failed".to_string())
}

/// 主画像を作る。max_pixel を指定すると、埋め込みのプレビューがあればそれを縮小して使う（高速）。
/// 指定しなければ元の画像から等倍で作る。どちらも Exif の向きを反映する
fn primary_image(source: &Cf, max_pixel: Option<u32>) -> Result<Cf, String> {
    let index = unsafe { CGImageSourceGetPrimaryImageIndex(source.0) };

    let size = max_pixel.and_then(|px| number_i64(px as i64));
    let options = unsafe {
        match &size {
            Some(size) => dictionary(&[
                (
                    kCGImageSourceCreateThumbnailFromImageIfAbsent,
                    kCFBooleanTrue,
                ),
                (kCGImageSourceCreateThumbnailWithTransform, kCFBooleanTrue),
                (kCGImageSourceThumbnailMaxPixelSize, size.0),
            ]),
            None => dictionary(&[
                (kCGImageSourceCreateThumbnailFromImageAlways, kCFBooleanTrue),
                (kCGImageSourceCreateThumbnailWithTransform, kCFBooleanTrue),
            ]),
        }
    }
    .ok_or("CFDictionaryCreate failed")?;

    Cf::new(unsafe { CGImageSourceCreateThumbnailAtIndex(source.0, index, options.0) })
        .ok_or_else(|| "CGImageSourceCreateThumbnailAtIndex failed".to_string())
}

/// ICNS のように複数の解像度を持つ画像から、いちばん大きいものを選ぶ
fn largest_image(source: &Cf) -> Result<Cf, String> {
    let count = unsafe { CGImageSourceGetCount(source.0) };
    (0..count)
        .filter_map(|i| {
            Cf::new(unsafe { CGImageSourceCreateImageAtIndex(source.0, i, std::ptr::null()) })
        })
        .max_by_key(|image| unsafe { CGImageGetWidth(image.0) * CGImageGetHeight(image.0) })
        .ok_or_else(|| "no image in the source".to_string())
}

fn encode(image: &Cf, output: Output) -> Result<Vec<u8>, String> {
    let data = Cf::new(unsafe { CFDataCreateMutable(std::ptr::null(), 0) })
        .ok_or("CFDataCreateMutable failed")?;
    let uti = match output {
        Output::Jpeg => string(c"public.jpeg"),
        Output::Png => string(c"public.png"),
    }
    .ok_or("CFStringCreateWithCString failed")?;

    let dest =
        Cf::new(unsafe { CGImageDestinationCreateWithData(data.0, uti.0, 1, std::ptr::null()) })
            .ok_or("CGImageDestinationCreateWithData failed")?;

    let quality = number_f64(0.9).ok_or("CFNumberCreate failed")?;
    let properties = match output {
        Output::Jpeg => Some(
            dictionary(&[(
                unsafe { kCGImageDestinationLossyCompressionQuality },
                quality.0,
            )])
            .ok_or("CFDictionaryCreate failed")?,
        ),
        Output::Png => None,
    };

    unsafe {
        CGImageDestinationAddImage(
            dest.0,
            image.0,
            properties.as_ref().map_or(std::ptr::null(), |p| p.0),
        );
        if !CGImageDestinationFinalize(dest.0) {
            return Err("CGImageDestinationFinalize failed".to_string());
        }

        let len = CFDataGetLength(data.0) as usize;
        let ptr = CFDataGetBytePtr(data.0);
        Ok(std::slice::from_raw_parts(ptr, len).to_vec())
    }
}

/// 写真（RAW など）を JPEG に変換する。max_pixel を指定するとサムネイル用に縮小する
pub fn convert_photo(path: &Path, max_pixel: Option<u32>) -> Result<Vec<u8>, String> {
    let source = open_source(path)?;
    let image = primary_image(&source, max_pixel)?;
    encode(&image, Output::Jpeg)
}

/// アイコン（ICNS）を、いちばん大きい解像度の PNG に変換する
pub fn convert_icon(path: &Path) -> Result<Vec<u8>, String> {
    let source = open_source(path)?;
    let image = largest_image(&source)?;
    encode(&image, Output::Png)
}
