//! Print the ID3v2 frames of one or more files.
//!
//! ```sh
//! cargo run --example read -- path/to/file.mp3
//! ```

use std::env;
use std::process::ExitCode;

use errai::content::Content;
use errai::error::Result;
use errai::frame::DynFrame;
use errai::id3v2::Tag;

fn print_tag(path: &str) -> Result<()> {
  let tag: Tag = Tag::from_path(path)?;

  println!("{path} ({:?})", tag.header().version());

  for frame in tag.frames() {
    let frame: DynFrame<'_> = frame?;
    let label: &str = frame.identifier_str();

    match frame.decode()? {
      Content::Text(data) => println!("  {label}: {}", data.text_content()),
      Content::Txxx(data) => {
        println!(
          "  {label}: {} = {}",
          data.text_summary(),
          data.text_details()
        )
      }
      Content::Comm(data) => println!("  {label}: {}", data.text_details()),
      Content::Wxxx(data) => println!("  {label}: {}", data.url()),
      Content::Apic(data) => println!(
        "  {label}: {:?} {:?} ({} bytes)",
        data.picture_type(),
        data.image_format(),
        data.picture_data().len(),
      ),
      other => println!("  {label}: {other:?}"),
    }
  }

  Ok(())
}

fn main() -> ExitCode {
  let paths: Vec<String> = env::args().skip(1).collect();

  if paths.is_empty() {
    eprintln!("usage: read <file>...");
    return ExitCode::FAILURE;
  }

  let mut status: ExitCode = ExitCode::SUCCESS;

  for path in &paths {
    if let Err(error) = print_tag(path) {
      eprintln!("{path}: {error}");
      status = ExitCode::FAILURE;
    }
  }

  status
}
