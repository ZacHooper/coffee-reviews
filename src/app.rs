use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Hello, World!"</h1>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/pkg/output.css"/>
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=Home/>
            </Routes>
        </Router>
    }
}
