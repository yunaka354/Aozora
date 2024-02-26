use crate::models::{Embed, Feed, Reason};
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
            // embed content if it is available
            {
                if let Some(embed) = &props.feed.post.embed {
                    html! {
                        <div class="px-6 py-4">
                            <EmbedContent embed={embed.clone()}/>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            // post status
            <div class="flex justify-between px-6 py-4">
                <div class="flex items-center">
                    <i class="fa-regular fa-message"></i>
                    <span class="px-2">{&props.feed.post.reply_count}</span>
                </div>
                <div class="flex items-center">
                    <i class="fa-solid fa-retweet"></i>
                    <span class="px-2">{&props.feed.post.repost_count}</span>
                </div>
                <div class="flex items-center">
                    <i class="fa-regular fa-heart"></i>
                    <span class="px-2">{&props.feed.post.like_count}</span>
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

#[derive(Properties, Clone, PartialEq)]
pub struct EmbedContentProps {
    pub embed: Embed,
}

// embed content if it is available
#[function_component(EmbedContent)]
pub fn embed_content(props: &EmbedContentProps) -> Html {
    html! {
        match &props.embed.images {
            Some(images) => {
                html! {
                    <img src={images[0].thumb.clone()} alt="Embed Content" />
                }
            }
            None => {
                html! {}
            }
        }
    }
}
