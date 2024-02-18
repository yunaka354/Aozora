mod components;
use components::card::PostCard;
mod models;
use models::Timeline;

use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::Value;


#[function_component]
fn App() -> Html {
    let timeline = use_state(|| None);
    {
        let timeline = timeline.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/timeline")
                    .send()
                    .await
                    .expect("Failed to fetch timeline");
                let text = response.text().await.expect("Failed to parse response");
                let json_data: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
                let timeline_model: Timeline = serde_json::from_value(json_data).expect("Failed to parse JSON");
                timeline.set(Some(timeline_model));
            });
        })
    }
    html! {
        <div class="max-w-lg mx-auto">
            <div class="fixed top-0 left-0 m-4">
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    { "Tweet" }
                </button>
            </div>
            <div class="flex items-center justify-center min-h-screen">
                {
                    match (*timeline).clone() {
                        Some(timeline) => {
                            html! {
                                <div>
                                    {
                                        for timeline.feed.iter().map(|feed| {
                                            html! {
                                                <PostCard
                                                    avatar={feed.post.author.avatar.clone()}
                                                    username={feed.post.author.display_name.clone()}
                                                    content={feed.post.record.text.clone()}
                                                    posted_on={feed.post.record.created_at.clone()}
                                                />
                                            }
                                        })
                                    }
                                </div>
                            }
                        }
                        None => {
                            html! {
                                <div class="text-center mt-8">
                                    <div class="inline-block animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
                                </div>
                            }
                        }
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}