extern crate http;

use std::vec::Vec;
use http::headers::content_type::MediaType;

pub fn content_type_from_ext(extension: &str) -> MediaType {
    let pieces: ~[&str] = mimetype_from_ext(extension).split('/').collect();
    MediaType {
        type_: pieces[0].to_owned(),
        subtype: pieces[1].to_owned(),
        parameters: Vec::new(),
    }
}

fn mimetype_from_ext<'a>(extension: &str) -> ~str {
    match extension {
        // application
        "eot"              => ~"application/vnd.ms-fontobject",
        "js"               => ~"application/javascript",
        "json"             => ~"application/json",
        "pdf"              => ~"application/pdf",
        "ttf"              => ~"application/x-font-ttf",
        "woff"             => ~"application/font-woff",
        "xml"              => ~"application/xml",

        // audio
        "mp3"              => ~"audio/mpeg",

        // images
        "bmp"              => ~"image/bmp",
        "gif"              => ~"image/gif",
        "jpg" | "jpeg"    => ~"image/jpeg",
        "png"              => ~"image/png",
        "svg" | "svgz"    => ~"image/svg+xml",
        "tif" | "tiff"    => ~"image/tiff",

        // text
        "css"              => ~"text/css",
        "htm" | "html"    => ~"text/html",
        "txt" | "text"    => ~"text/plain",
        // unknown
        _                   => ~"text/plain",
    }
}