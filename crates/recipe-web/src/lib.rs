#[derive(Debug)]
#[cfg_attr(feature = "embed", derive(rust_embed::RustEmbed))]
#[cfg_attr(feature = "embed", folder = "$CARGO_MANIFEST_DIR/dist")]
#[cfg_attr(feature = "embed", prefix = "assets/")]
pub struct Assets;
