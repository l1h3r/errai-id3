# errai

[![github][shield-img-github]][shield-url-github]
[![build status][shield-img-ci]][shield-url-ci]

A zero-copy ID3v2 tag reader for Rust.

**errai** reads ID3v2.2, ID3v2.3 and ID3v2.4 tags and decodes their frames into typed values. Frame content borrows from the tag buffer wherever the encoding allows it, and any decoded value can be converted to an owned `'static` value with `IntoOwned`.

## Features

- **Three tag versions** - ID3v2.2, ID3v2.3 and ID3v2.4 headers, extended headers and frames.
- **Typed frames** - Text, URLs, comments, lyrics, pictures, chapters, play counters, event timing codes and more.
- **Zero-copy decoding** - ASCII and UTF-8 text and binary data borrow from the tag buffer.
- **Text encodings** - ISO-8859-1, UTF-16 with BOM, UTF-16BE and UTF-8.
- **Lenient by default** - Unknown or unimplemented frames are returned as raw data instead of failing the tag.
- **Compressed frames** - ZLIB decompression behind the default `zlib` feature.

## Usage

The crate is not published to crates.io. Add it as a git dependency:

```toml
[dependencies]
errai = { git = "https://github.com/l1h3r/errai-id3" }
```

## Example

```rust,no_run
use errai::content::Content;
use errai::error::Result;
use errai::frame::DynFrame;
use errai::id3v2::Tag;

fn main() -> Result<()> {
  // Read the ID3v2 tag at the start of the file.
  let tag: Tag = Tag::from_path("song.mp3")?;

  for frame in tag.frames() {
    let frame: DynFrame<'_> = frame?;

    // Decode each frame and print the text frames.
    if let Content::Text(text) = frame.decode()? {
      println!("{}: {}", frame.identifier_str(), text.text_content());
    }
  }

  Ok(())
}
```

The `read` example prints every frame of the files it is given:

```sh
cargo run --example read -- crates/parser/tests/fixtures/antimatter15.mp3
```

## Supported Frames

| Group | Frames |
| --- | --- |
| Text | `T***`, `TXXX`, `IPLS` |
| URLs | `W***`, `WXXX` |
| Comments and lyrics | `COMM`, `USLT`, `SYLT` |
| Media | `APIC`, `GEOB`, `MCDI`, `MLLT` |
| Timing | `ETCO`, `SYTC`, `POSS` |
| Volume and effects | `EQUA`, `RVAD`, `RVA2`, `RVRB` |
| Playback | `PCNT`, `POPM`, `RBUF` |
| Commerce and rights | `COMR`, `OWNE`, `USER` |
| Identification | `UFID`, `PRIV`, `LINK`, `AENC`, `ENCR`, `GRID` |
| Addenda | `CHAP`, `CTOC`, `ATXT` |

ID3v2.2 uses the three-character equivalents of these frames.

## Limitations

- Tags are read-only. There is no writer.
- Only a tag at the start of the input is read. Appended tags and ID3v1 tags are not.
- `ASPI`, `EQU2`, `SEEK`, `SIGN`, `CRM` and the `RVA2`/`RVAD` channel data are returned as raw data.
- Per-frame unsynchronisation in ID3v2.4, unsynchronised extended headers, encrypted frames and frames embedded in `CTOC` return an `Unsupported` error.

## Layout

| Path | Contents |
| --- | --- |
| `src/` | The `errai` crate, which re-exports `parser`. |
| `crates/parser/` | The tag and frame parser. |
| `crates/derive/` | The `Frame` derive macro used to generate frame decoders and accessors. |
| `docs/` | Copies of specifications that are no longer hosted. |

## References

- ID3v2.2 - <https://web.archive.org/web/20161124001116/https://id3.org/id3v2-00>
- ID3v2.3 - <https://web.archive.org/web/20161119235516/https://id3.org/id3v2.3.0>
- ID3v2.4 structure - <https://web.archive.org/web/20161211074636/https://id3.org/id3v2.4.0-structure>
- ID3v2.4 frames - <http://www.unixgods.org/Ruby/ID3/docs/id3v2.4.0-frames.html>
- ID3v2.3 (mutagen mirror) - <https://mutagen-specs.readthedocs.io/en/latest/id3/id3v2.3.0.html>
- Chapter frame addendum - [`docs/ID3v2 Chapter Frame Addendum.md`][chapters], originally <https://web.archive.org/web/20221115030430/http://id3.org/id3v2-chapters-1.0>
- Accessibility addendum - <https://web.archive.org/web/20221224181127/http://id3.org/id3v2-accessibility-1.0>
- ID3v1 - <https://web.archive.org/web/20170101224120/http://id3.org/ID3v1>
- ID3 developer information - <https://web.archive.org/web/20221015060936/https://id3.org/Developer%20Information>

Other Rust ID3 libraries: [`id3`][rust-id3], [`lofty`][lofty], [`rtag`][rtag] and `musikr`.

The test fixture `antimatter15.mp3` is a sample file by antimatter15 that is widely used in the test suites of ID3 libraries.

<br>

#### License

<sup>
  Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
  Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>

[//]: # (links)

[chapters]: docs/ID3v2%20Chapter%20Frame%20Addendum.md
[rust-id3]: https://github.com/polyfloyd/rust-id3
[lofty]: https://github.com/Serial-ATA/lofty-rs
[rtag]: https://github.com/freestrings/rtag

[//]: # (badges)

[shield-url-github]: https://github.com/l1h3r/errai-id3
[shield-img-github]: https://img.shields.io/badge/github-l1h3r/errai--id3-main?style=flat-square&logo=github

[shield-url-ci]: https://github.com/l1h3r/errai-id3/actions/workflows/ci.yml?query=branch:main
[shield-img-ci]: https://img.shields.io/github/actions/workflow/status/l1h3r/errai-id3/ci.yml?style=flat-square
