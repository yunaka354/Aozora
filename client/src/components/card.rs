use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostCardProps {
    pub username: String,
    pub content: String,
    pub posted_on: String,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    html! {
        <div class="bg-white p-6 rounded-lg shadow-lg mb-2">
            <h2 class="text-xl font-bold mb-2">{&props.username}</h2>
            <p class="text-gray-700">{&props.content}</p>
        <div class="text-gray-600 text-sm mt-4">{&props.posted_on}</div>
    </div>
    }
}

