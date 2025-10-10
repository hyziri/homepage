//! Guide pages related specifically to Autumn
//!
//! These guides are split between highsec, nullsec, and shared:
//!
//! - [`highsec`]: Guides related to Autumn's highsec division, `Autumn Inc.`
//! - [`nullsec`]: Guides related to Autumn's nullsec division, `The Order of Autumn`
//! - [`shared`]: Guides shared between highsec & nullsec
//!
//! ## Shared Guide URLs
//!
//! Shared guide URLs will be prepended with either `/highsec` or `/nullsec` depending on the category of guides the
//! user is focused on such as `Autumn Highsec` or `Autumn Nullsec`. Despite the difference in URL, they still
//! share the same source file.
//!
//! This is done so that breadcrumbs at the top of guides & URLs don't jump around and cause confusion such as being
//! on a nullsec-specific guide `/autumn/nullsec/services/srp` and jumping to a shared guide `/autumn/services/buyback`.
//! It makes URLs more predicatable.

pub mod highsec;
pub mod nullsec;
pub mod shared;
