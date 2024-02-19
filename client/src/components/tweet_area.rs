use yew::prelude::*;

#[function_component(TweetArea)]
pub fn tweet_area() -> Html {
    html! {
        <>
            <h2 class="text-lg font-semibold mb-4">{"New Tweet"}</h2>
            <div class="me-4 mb-4">
                <textarea class="w-full p-4 border border-gray-300 rounded" rows="4" placeholder="What's happening?"></textarea>
            </div>
            <div>
                <TweetButton />
            </div>
        </>
    }
}

#[function_component(TweetButton)]
pub fn tweet_button() -> Html {
    html! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            { "Tweet" }
        </button>
    }
}
