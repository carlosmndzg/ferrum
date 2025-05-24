# Ferrum: A Minimal Web Browser Layout Engine in Rust

Ferrum is a minimal web browser layout engine developed from scratch in Rust as part of a final year Computer Engineering degree project.  
It parses HTML and CSS, builds a DOM and layout tree, and renders the result visually.  
The project focuses on the core layout and rendering pipeline, omitting JavaScript and networking.

## Features

- **HTML**: Parses any HTML file, handles malformed HTML (e.g., missing `<html>` tag).
- **CSS**: Supports inline styles, `<style>` tags, and external stylesheets via `<link>`.  
  Universal, simple (element, class, id), and multiple selectors per rule are supported.  
  Property values: px, unitless, percentages, and keywords (integers only for numbers).  
  Panics on invalid CSS input.
- **Style Module**:
  - UA stylesheet support
  - Inheritance (not the `inherit` keyword)
  - Cascade (author and UA origins, no `!important`)
  - Initial values
- **Layout Module**:
  - Replaced elements (`<img>`, block-level only)
  - Only `static` positioning (no `position` property)
  - Supports `display: block`, `inline`, and `none` (normal flow)
  - Block-level box model
  - No vertical margin collapse
  - No horizontal margin/padding for inline elements
  - No block nodes inside inline nodes
  - Anonymous block support
  - Block and Inline Formatting Contexts
- **Window resize supported**

### Supported CSS Properties

- `background-color` (rgb, color keywords)
- `border-width` (keywords: thin, medium, thick; px)
- `border-style` (none, hidden, solid)
- `border-color` (rgb, color keywords)
- `border`
- `color` (rgb, color keywords)
- `display` (block, inline, none)
- `font-size` (px)
- `font-weight` (normal, bold, 1–1000)
- `height` (px, %, auto)
- `line-height` (unitless, integer only)
- `margin-bottom`, `margin-left`, `margin-right`, `margin-top`, `margin` (px, %, auto)
- `padding-bottom`, `padding-left`, `padding-right`, `padding-top`, `padding` (px, %)
- `text-align` (left, right, center, justify)
- `width` (px, %, auto)

## How to Build and Run

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

### Build

```sh
cargo build --release
```

### Run

```sh
# Run the compiled binary (from the project root)
./target/release/ferrum testing/tc8_layout.html

# Or, run directly with cargo
cargo run --release -- testing/tc8_layout.html
```
