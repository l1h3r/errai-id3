use std::collections::BTreeMap;

use parser::content::Content;
use parser::frame::DynFrame;
use parser::id3v2::Tag;
use parser::types::Version;

const FIXTURE: &str = concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/tests/fixtures/antimatter15.mp3"
);

fn decode_all(tag: &Tag) -> BTreeMap<String, Content<'_>> {
  let mut output: BTreeMap<String, Content<'_>> = BTreeMap::new();

  for frame in tag.frames() {
    let frame: DynFrame<'_> = frame.unwrap();
    let label: String = frame.identifier_str().to_owned();

    output.insert(label, frame.decode().unwrap());
  }

  output
}

fn text(frames: &BTreeMap<String, Content<'_>>, name: &str) -> String {
  match &frames[name] {
    Content::Text(inner) => inner.text_content().to_string(),
    other => panic!("expected text frame for {name}, got {other:?}"),
  }
}

#[test]
fn read_fixture_header() {
  let tag: Tag = Tag::from_path(FIXTURE).unwrap();

  assert_eq!(tag.header().version(), Version::ID3v24);
}

#[test]
fn read_fixture_frames() {
  let tag: Tag = Tag::from_path(FIXTURE).unwrap();
  let frames: BTreeMap<String, Content<'_>> = decode_all(&tag);

  assert_eq!(frames.len(), 14);

  assert_eq!(text(&frames, "TIT2"), "Uh, something");
  assert_eq!(text(&frames, "TPE1"), "antimatter15");
  assert_eq!(text(&frames, "TALB"), "SUPER MAGICALNESS");
  assert_eq!(text(&frames, "TRCK"), "42");
  assert_eq!(text(&frames, "TCON"), "AWESOMENESS");
  assert_eq!(text(&frames, "TDRC"), "2010");

  let Content::Comm(comm) = &frames["COMM"] else {
    panic!("expected COMM frame");
  };

  assert_eq!(
    comm.text_details(),
    "this is the most awesomeest thing EVAH"
  );

  let Content::Wxxx(wxxx) = &frames["WXXX"] else {
    panic!("expected WXXX frame");
  };

  assert_eq!(wxxx.url(), "http://antimatter15.com");

  let Content::Apic(apic) = &frames["APIC"] else {
    panic!("expected APIC frame");
  };

  assert_eq!(apic.picture_data().len(), 30373);
  assert!(apic.picture_data().as_ref().starts_with(&[0xFF, 0xD8]));
}

#[test]
fn read_fixture_v23_frame_in_v24_tag() {
  // TYER is an ID3v2.3 frame. It is not valid in an ID3v2.4 tag and is
  // returned as raw data.
  let tag: Tag = Tag::from_path(FIXTURE).unwrap();
  let frames: BTreeMap<String, Content<'_>> = decode_all(&tag);

  assert!(matches!(frames["TYER"], Content::Unkn(_)));
}

#[test]
fn read_missing_file() {
  assert!(Tag::from_path("does/not/exist.mp3").is_err());
}

#[test]
fn read_non_id3_data() {
  let error = Tag::from_reader(&b"not an id3 tag"[..]).unwrap_err();

  assert!(error.to_string().starts_with("invalid tag field"));
}
