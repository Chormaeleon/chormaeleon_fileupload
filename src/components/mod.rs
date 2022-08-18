pub(crate) mod admin_only;
pub(crate) mod delete_modal;
pub(crate) mod iframe;
pub(crate) mod jwt_context;
pub(crate) mod material;
pub(crate) mod modal;
pub(crate) mod progress;
pub(crate) mod project;
pub(crate) mod submission;
pub(crate) mod upload;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum PlaceholderOrContent {
    Placeholder(String),
    Content(String),
}
