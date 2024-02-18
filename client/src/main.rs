mod components;
use components::timeline::Timeline;
use components::tweet_button::TweetButton;
mod models;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="max-w-lg mx-auto">
            <div class="fixed top-0 left-0 m-4">
                <TweetButton />
            </div>
            <div class="flex items-center justify-center min-h-screen">
                <Timeline />
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
