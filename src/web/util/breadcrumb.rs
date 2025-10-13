use crate::web::{model::breadcrumb::Breadcrumb, util::format::format_title, Route};

/// Converts a URL path into segment pairs with a title & href for breadcrumbs
///
/// Input: `/guides/join-autumn`
/// Output href: `/guides`, `/guides/join-autumn
/// Output names: `Guides`, `Join Autumn`
pub fn path_to_breadcrumbs(path: String) -> Vec<Breadcrumb> {
    // Remove fragment (`#...`) first
    let without_fragment = match path.find('#') {
        Some(idx) => &path[..idx],
        None => &path[..],
    };

    // Remove leading/trailing slashes so split is predictable
    let trimmed = without_fragment.trim_matches('/');

    if trimmed.is_empty() {
        Vec::new()
    } else {
        let mut crumbs = Vec::new();
        let mut accumulated = String::new();
        for segment in trimmed.split('/') {
            if segment.is_empty() {
                continue;
            }

            // accumulate with a leading slash
            accumulated.push('/');
            accumulated.push_str(segment);

            let route: Route = accumulated.parse().unwrap();

            crumbs.push(Breadcrumb {
                name: format_title(segment),
                route,
            });
        }
        crumbs
    }
}
