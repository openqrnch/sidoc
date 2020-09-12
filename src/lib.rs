//! A library for generating indented scoped text documents consisting of
//! reusable parts.
//!
//! There are three major steps involved in using idoc:
//! - Creating document(s)
//! - Adding document(s) to a render context
//! - Render output
//!
//!# Simple example
//! ```
//! use std::sync::Arc;
//! use sidoc::{Builder, RenderContext};
//!
//! fn create_doc() {
//!   // Use a builder to generate document
//!   let mut bldr = Builder::new();
//!   bldr.line("<!DOCTYPE html>");
//!   bldr.scope("<html>", Some("</html>")).exit();
//!   let doc = bldr.build().unwrap();
//!
//!   // Create a render context, add document to it
//!   let mut r = RenderContext::new();
//!   r.doc("root", Arc::new(doc));
//!
//!   // Render the output
//!   let buf = r.render("root").unwrap();
//!   assert_eq!(buf, "<!DOCTYPE html>\n<html>\n</html>\n");
//! }
//! ```
//!
//! # Reference example
//! ```
//! use std::sync::Arc;
//! use sidoc::{Builder, RenderContext};
//!
//! fn create_doc() {
//!   // Use a builder to generate root document
//!   let mut bldr = Builder::new();
//!   bldr.scope("<html>", Some("</html>")).exit();
//!   let doc = bldr.build().unwrap();
//!
//!   // Use a builder to generate sub document
//!   let mut bldr = Builder::new();
//!   bldr.scope("<head>", Some("</head>")).exit();
//!   let subdoc = bldr.build().unwrap();
//!
//!   // Create a render context, add documents to it
//!   let mut r = RenderContext::new();
//!   r.doc("root", Arc::new(doc));
//!   r.doc("head", Arc::new(subdoc));
//!
//!   // Render the output
//!   let buf = r.render("root").unwrap();
//!   assert_eq!(buf, "<html>\n  <head>\n  </head>\n</html>\n");
//! }
//! ```

mod builder;
mod doc;
mod err;
mod render;

pub use builder::Builder;
pub use doc::Doc;
pub use err::Error;
pub use render::RenderContext;

pub(crate) enum Node {
  Line(String),
  BeginScope(String),
  EndScope(Option<String>),
  OptRef(String),
  ReqRef(String)
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
