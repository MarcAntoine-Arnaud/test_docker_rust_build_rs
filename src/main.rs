
pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
  println!("{:?}", built_info::PKG_NAME);
}
