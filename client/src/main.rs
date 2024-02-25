mod components;
use components::header::Header;
use components::timeline::Timeline;
use components::tweet_area::TweetArea;
mod models;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <div class={"wrapper flex bg-gray-100 pt-16"}>
                    <div class="w-1/3">
                        <TweetArea />
                    </div>
                    <div class="w-1/3">
                        <div class="flex items-center min-h-screen">
                            <Timeline />
                        </div>
                    </div>
                    <div class="w-1/3"></div>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
