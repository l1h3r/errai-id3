use parser::content::Content;
use parser::error::ErrorKind;
use parser::error::Result;
use parser::frame::DynFrame;
use parser::types::Slice;
use parser::types::Version;

fn frame(version: Version, bytes: &[u8]) -> DynFrame<'_> {
  DynFrame::from_slice(version, Slice::new(bytes))
    .unwrap()
    .unwrap()
}

fn text(content: &Content<'_>) -> String {
  match content {
    Content::Text(inner) => inner.text_content().to_string(),
    _ => panic!("expected text frame, got {content:?}"),
  }
}

#[test]
fn v24_data_length_indicator_without_compression() {
  // TIT2, size 10, flags 0x0001 (data length indicator), length 5, "\0Hello".
  let bytes: &[u8] = b"TIT2\x00\x00\x00\x0A\x00\x01\x00\x00\x00\x06\x00Hello";
  let content: Content<'_> = frame(Version::ID3v24, bytes).decode().unwrap();

  assert_eq!(text(&content), "Hello");
}

#[test]
fn v24_encrypted_frame_is_unsupported() {
  // TIT2, size 7, flags 0x0004 (encryption), method 0x80, "\0Hello".
  let bytes: &[u8] = b"TIT2\x00\x00\x00\x07\x00\x04\x80\x00Hello";
  let error = frame(Version::ID3v24, bytes).decode().unwrap_err();

  assert!(matches!(error.kind(), ErrorKind::Unsupported(_)));
}

#[test]
fn v23_encrypted_frame_is_unsupported() {
  // TIT2, size 7, flags 0x0040 (encryption), method 0x80, "\0Hello".
  let bytes: &[u8] = b"TIT2\x00\x00\x00\x07\x00\x40\x80\x00Hello";
  let error = frame(Version::ID3v23, bytes).decode().unwrap_err();

  assert!(matches!(error.kind(), ErrorKind::Unsupported(_)));
}

#[test]
fn v24_descriptor_smaller_than_extra_data() {
  // TIT2, size 1, flags 0x0001, but the length field alone needs 4 bytes.
  let bytes: &[u8] = b"TIT2\x00\x00\x00\x01\x00\x01\x00\x00\x00\x00";

  // Decoding must return, either with a value or an error, without panicking
  // on the size underflow.
  let _outcome: Result<Content<'_>> = frame(Version::ID3v24, bytes).decode();
}
