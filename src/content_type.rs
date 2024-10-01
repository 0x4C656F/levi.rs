use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ContentType {
    TextPlain,
    TextHtml,
    TextCss,
    ApplicationJson,
    ApplicationJavascript,
    ApplicationXml,
    ApplicationOctetStream,
    MultipartFormData,
    ApplicationFormUrlEncoded,
    ImagePng,
    ImageJpeg,
    ImageGif,
    AudioMpeg,
    AudioOgg,
    VideoMp4,
    VideoWebm,
    Unknown,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::TextHtml => "text/html",
            ContentType::TextCss => "text/css",
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationJavascript => "application/javascript",
            ContentType::ApplicationXml => "application/xml",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::ApplicationFormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::ImagePng => "image/png",
            ContentType::ImageJpeg => "image/jpeg",
            ContentType::ImageGif => "image/gif",
            ContentType::AudioMpeg => "audio/mpeg",
            ContentType::AudioOgg => "audio/ogg",
            ContentType::VideoMp4 => "video/mp4",
            ContentType::VideoWebm => "video/webm",
            ContentType::Unknown => "unknown",
        }
    }
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text/plain" => Ok(ContentType::TextPlain),
            "text/html" => Ok(ContentType::TextHtml),
            "text/css" => Ok(ContentType::TextCss),
            "application/json" => Ok(ContentType::ApplicationJson),
            "application/javascript" => Ok(ContentType::ApplicationJavascript),
            "application/xml" => Ok(ContentType::ApplicationXml),
            "application/octet-stream" => Ok(ContentType::ApplicationOctetStream),
            "multipart/form-data" => Ok(ContentType::MultipartFormData),
            "application/x-www-form-urlencoded" => Ok(ContentType::ApplicationFormUrlEncoded),
            "image/png" => Ok(ContentType::ImagePng),
            "image/jpeg" => Ok(ContentType::ImageJpeg),
            "image/gif" => Ok(ContentType::ImageGif),
            "audio/mpeg" => Ok(ContentType::AudioMpeg),
            "audio/ogg" => Ok(ContentType::AudioOgg),
            "video/mp4" => Ok(ContentType::VideoMp4),
            "video/webm" => Ok(ContentType::VideoWebm),
            _ => Ok(ContentType::Unknown),
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
