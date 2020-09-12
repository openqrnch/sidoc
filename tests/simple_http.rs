use std::sync::Arc;

use sidoc::{Builder, RenderContext};

#[test]
fn simple_html() {
  let mut bldr = Builder::new();

  bldr.line("<!DOCTYPE html>");
  bldr.scope("<html>", Some("</html>")).exit();

  let doc = bldr.build().unwrap();

  let mut r = RenderContext::new();

  r.doc("hello", Arc::new(doc));

  let buf = r.render("hello").unwrap();

  assert_eq!(buf, "<!DOCTYPE html>\n<html>\n</html>\n");
}


#[test]
fn simple_html_head() {
  let mut bldr = Builder::new();

  bldr
    .line("<!DOCTYPE html>")
    .scope("<html>", Some("</html>"))
    .scope("<head>", Some("</head>"))
    .exit()
    .exit();

  let doc = bldr.build().unwrap();

  let mut r = RenderContext::new();

  r.doc("root", Arc::new(doc));

  let buf = r.render("root").unwrap();

  assert_eq!(
    buf,
    "<!DOCTYPE html>\n<html>\n  <head>\n  </head>\n</html>\n"
  );
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
