use std::sync::Arc;

use sidoc::{Builder, Doc, RenderContext};

#[cfg(test)]
fn gen_ref() -> Doc {
  let mut bldr = Builder::new();

  bldr.scope("<head>", Some("</head>")).exit();

  bldr.build().unwrap()
}


#[test]
fn simple_ref() {
  let mut bldr = Builder::new();
  bldr
    .line("<!DOCTYPE html>")
    .scope("<html>", Some("</html>"))
    .optref("nonexistent")
    .reqref("the_head")
    .exit();

  let mut r = RenderContext::new();
  r.doc("the_head", Arc::new(gen_ref()));
  r.doc("the_root", Arc::new(bldr.build().unwrap()));

  let buf = r.render("the_root").unwrap();

  assert_eq!(
    buf,
    "<!DOCTYPE html>\n<html>\n  <head>\n  </head>\n</html>\n"
  );
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
