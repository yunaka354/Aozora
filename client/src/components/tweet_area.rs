use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Tweet {
    text: String,
}

#[function_component(TweetArea)]
pub fn tweet_area() -> Html {
    let tweet_text = use_state(|| String::new());
    let on_input = {
        let tweet_text = tweet_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            tweet_text.set(input.value());
        })
    };
    let button_click = {
        let tweet_text = tweet_text.clone();
        Callback::from(move |_| {
            let tweet_text = tweet_text.clone();

            // Serialize the tweet
            let json_data = match serde_json::to_string(&Tweet { text: (*tweet_text).clone() }) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Failed to serialize tweet: {}", err);
                    return;
                }
            };
            
            // Send the tweet to the server
            wasm_bindgen_futures::spawn_local(async move {
                let _response = Request::post("http://localhost:3000/tweet")
                    .header("Content-Type", "application/json")
                    .body(json_data)
                    .unwrap()
                    .send()
                    .await
                    .expect("Failed to send tweet");
            });
        })
    };

    html! {
        <>
            <h2 class="text-lg font-semibold mb-4">{"New Tweet"}</h2>
            <div class="me-4 mb-4">
                <textarea
                    oninput={on_input}
                    class="w-full p-4 border border-gray-300 rounded"
                    rows="4"
                    placeholder="What's happening?"
                    value={(*tweet_text).clone()}
                >
                </textarea>
            </div>
            <div>
                <TweetButton callback={button_click.clone()} />
            </div>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TweetButtonProps {
    callback: Callback<MouseEvent>,
}

#[function_component(TweetButton)]
pub fn tweet_button(TweetButtonProps{callback}: &TweetButtonProps) -> Html {
    html! {
        <button onclick={callback} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            { "Tweet" }
        </button>
    }
}
