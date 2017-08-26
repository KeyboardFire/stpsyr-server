mod prelude;

mod html;
pub use self::html::handle_html;

mod css;
pub use self::css::handle_css;

mod staticfile;
pub use self::staticfile::handle_static;
