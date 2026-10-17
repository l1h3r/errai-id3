use parser::content::Content;
use parser::error::ErrorKind;
use parser::types::Slice;
use parser::types::Version;

fn text(content: &Content<'_>) -> String {
  match content {
    Content::Text(inner) => inner.text_content().to_string(),
    _ => panic!("expected text frame, got {content:?}"),
  }
}

#[test]
fn latin1_text_is_converted_to_utf8() {
  // Encoding byte 0x00 (ISO-8859-1), followed by "Café" where é = 0xE9.
  let data: &Slice = Slice::new(b"\x00Caf\xE9");
  let content: Content<'_> = Content::decode(Version::ID3v23, "TIT2", data).unwrap();

  assert_eq!(text(&content), "Café");
}

#[test]
fn ascii_text_is_borrowed() {
  let data: &Slice = Slice::new(b"\x00Hello");
  let content: Content<'_> = Content::decode(Version::ID3v24, "TALB", data).unwrap();

  assert_eq!(text(&content), "Hello");
}

#[test]
fn utf8_text() {
  // Encoding byte 0x03 (UTF-8).
  let data: &Slice = Slice::new("\u{3}Café".as_bytes());
  let content: Content<'_> = Content::decode(Version::ID3v24, "TIT2", data).unwrap();

  assert_eq!(text(&content), "Café");
}

#[test]
fn unknown_frame_is_returned_as_raw_data() {
  let data: &Slice = Slice::new(b"\x01\x02\x03");
  let content: Content<'_> = Content::decode(Version::ID3v24, "ZZZZ", data).unwrap();

  let Content::Unkn(inner) = content else {
    panic!("expected unknown frame, got {content:?}");
  };

  assert_eq!(inner.binary_data().as_ref(), b"\x01\x02\x03");
}

#[test]
fn unimplemented_frame_is_returned_as_raw_data() {
  let data: &Slice = Slice::new(b"\xAA\xBB");
  let content: Content<'_> = Content::decode(Version::ID3v24, "SEEK", data).unwrap();

  assert!(matches!(content, Content::Unkn(_)));
}

#[test]
fn id3v1_version_is_rejected() {
  let data: &Slice = Slice::new(b"\x00Hello");
  let error = Content::decode(Version::ID3v11, "TIT2", data).unwrap_err();

  assert_eq!(error.kind(), ErrorKind::InvalidVersion);
}

#[test]
fn trailing_padding_is_accepted() {
  let data: &Slice = Slice::new(b"\x00\x00\x00\x00\x2A\x00\x00");
  let content = Content::decode(Version::ID3v24, "PCNT", data);

  assert!(content.is_ok());
}

#[test]
fn error_display_does_not_panic() {
  let data: &Slice = Slice::new(b"\x00Hello");
  let error = Content::decode(Version::ID3v11, "TIT2", data).unwrap_err();

  assert_eq!(error.to_string(), "invalid tag version");
}

#[test]
fn empty_utf16_text() {
  // Encoding byte 0x01 (UTF-16 with BOM) with no BOM and no text.
  let data: &Slice = Slice::new(b"\x01");
  let content: Content<'_> = Content::decode(Version::ID3v24, "TIT2", data).unwrap();

  assert_eq!(text(&content), "");
}

#[test]
fn truncated_utf16_text_is_an_error() {
  // Encoding byte 0x01 followed by a single byte, too short for a BOM.
  let data: &Slice = Slice::new(b"\x01\xFF");
  let error = Content::decode(Version::ID3v24, "TIT2", data).unwrap_err();

  assert_eq!(error.kind(), ErrorKind::InvalidFrameData);
}

#[test]
fn utf16_text_with_bom() {
  // Encoding byte 0x01, little-endian BOM, then "Hi".
  let data: &Slice = Slice::new(b"\x01\xFF\xFEH\x00i\x00");
  let content: Content<'_> = Content::decode(Version::ID3v24, "TIT2", data).unwrap();

  assert_eq!(text(&content), "Hi");
}
