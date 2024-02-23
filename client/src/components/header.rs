use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    // function to reload the page
    let reload = Callback::from(|_| {
        web_sys::window().unwrap().location().reload().unwrap();
    });
    html! {
        <nav class="fixed top-0 left-0 w-full bg-gray-800 text-white p-4">
            <h1 class="text-2xl font-bold" onclick={reload}>{"Aozora"}</h1>
        </nav>

    }
}
