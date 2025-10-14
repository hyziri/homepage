use crate::web::Route;

#[derive(PartialEq, Clone)]
pub struct HeaderLink {
    pub text: &'static str,
    pub route: Route,
}
