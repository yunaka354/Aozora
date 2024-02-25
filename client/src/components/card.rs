use crate::models::{Feed, Reason};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostCardProps {
    pub feed: Feed,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    html! {
        <div class="bg-white p-6 rounded-lg shadow-lg mb-2">
            // if the post is reposted, show the reposted mark
            {
                if let Some(reason) = &props.feed.reason {
                    html! {
                        <div class="px-6">
                            <RepostedMark reason={reason.clone()}/>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            // post content
            <div class="flex items-start px-6 py-4">
                <img class="w-16 h-16 rounded-full mr-4" src={props.feed.post.author.avatar.clone()} alt="User" />
                <div>
                    <h2 class="text-xl font-bold mb-2">
                        <a href={format!("https://bsky.app/profile/{}", props.feed.post.author.handle)} target="_blank">{&props.feed.post.author.display_name}</a>
                    </h2>
                    <p class="text-gray-700">{&props.feed.post.record.text}</p>
                    <div class="text-gray-600 text-sm mt-4">{&props.feed.post.record.created_at}</div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct RepostedMarkProps {
    pub reason: Reason,
}

// reposted annotation above the post if it is reposted
#[function_component(RepostedMark)]
pub fn reposted_mark(props: &RepostedMarkProps) -> Html {
    html! {
        <p class="text-gray-700">
            <i class="fa-solid fa-retweet"></i>
            <span class="px-2">
                {"Reposted by "}
                <a class="font-bold" href={format!("https://bsky.app/profile/{}", props.reason.by.handle)} target="_blank">
                    {&props.reason.by.display_name}
                </a>
            </span>
        </p>
    }
}
