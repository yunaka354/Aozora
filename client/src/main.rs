use yew::prelude::*;

mod components;
use components::card::{PostCard, PostCardProps};

#[function_component]
fn App() -> Html {
    let cards = [
        PostCardProps {
            username: "User 1".to_string(),
            content: "This is the content of the tweet. It might include hashtags, mentions, and links.".to_string(),
            posted_on: "Jan 1, 2024".to_string(),
        },
        PostCardProps {
            username: "User 2".to_string(),
            content: "Here's another example tweet, showcasing how easy it is to create these cards with Tailwind CSS.".to_string(),
            posted_on: "Jan 2, 2024".to_string(),
        },
    ];
    html! {
        <div class="max-w-lg mx-auto">
            {
                for cards.iter().map(|props| {
                    html! {
                        <PostCard username={props.username.clone()} content={props.content.clone()} posted_on={props.posted_on.clone()} />
                    }
                })
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}