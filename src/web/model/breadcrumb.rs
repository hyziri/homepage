use crate::web::Route;

#[derive(Clone, PartialEq)]
pub struct Breadcrumb {
    pub name: String,
    pub route: Route,
}
