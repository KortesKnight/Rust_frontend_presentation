use yew_router::Routable;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Main,
    #[at("/fibonacci")]
    Fibonacci,
    #[not_found]
    #[at("/404")]
    NotFound,
}