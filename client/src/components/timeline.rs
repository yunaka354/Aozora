use crate::components::card::PostCard;
use crate::models;
use gloo_net::http::Request;
use serde_json::Value;
use yew::prelude::*;

#[function_component(Timeline)]
pub fn timeline() -> Html {
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
                let timeline_model: models::Timeline =
                    serde_json::from_value(json_data).expect("Failed to parse JSON");
                timeline.set(Some(timeline_model));
            });
        })
    }

    html! {
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
                                            reason={feed.reason.clone()}
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
    }
}
