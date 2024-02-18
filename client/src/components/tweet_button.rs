use yew::prelude::*;

#[function_component(TweetButton)]
pub fn tweet_button() -> Html {
    html! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            { "Tweet" }
        </button>
    }
}
