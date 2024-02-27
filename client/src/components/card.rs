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
            <RepostedMark reason={props.feed.reason.clone()}/>
            // post content
            <PostContent feed={props.feed.clone()}/>
            // embed content if it is available
            <EmbedContent embed={props.feed.post.embed.clone()}/>
            // footer
            <PostCardFooter feed={props.feed.clone()}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct RepostedMarkProps {
    pub reason: Option<Reason>,
}

// reposted annotation above the post if it is reposted
#[function_component(RepostedMark)]
pub fn reposted_mark(props: &RepostedMarkProps) -> Html {
    if let Some(reason) = &props.reason {
        html! {
            <div class="px-6">
                <p class="text-gray-700">
                    <i class="fa-solid fa-retweet"></i>
                    <span class="px-2">
                        {"Reposted by "}
                        <a class="font-bold" href={format!("https://bsky.app/profile/{}", reason.by.handle)} target="_blank">
                            {reason.by.display_name.clone()}
                        </a>
                    </span>
                </p>
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(PostContent)]
pub fn post_content(props: &PostCardProps) -> Html {
    html! {
        <div class="flex items-start px-6 py-4">
            <img class="w-16 h-16 rounded-full mr-4" src={props.feed.post.author.avatar.clone()} alt="User" />
            <div>
                <h2 class="text-xl font-bold mb-2">
                    <a href={format!("https://bsky.app/profile/{}", props.feed.post.author.handle)} target="_blank">{&props.feed.post.author.display_name}</a>
                </h2>
                // overflow-wrap: anywhere is required for edge cases where the text has too many special characters (i.e. exclamations, etc.)
                <div class="text-gray-700" style="overflow-wrap: anywhere;">{&props.feed.post.record.text}</div>
                <div class="text-gray-600 text-sm mt-4">{&props.feed.post.record.created_at}</div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct EmbedContentProps {
    pub embed: Option<Embed>,
}

// embed content if it is available
#[function_component(EmbedContent)]
pub fn embed_content(props: &EmbedContentProps) -> Html {
    if let Some(embed) = &props.embed {
        html! {
            <div class="px-6 py-4">
            {
                if let Some(images) = &embed.images {
                    html! {
                        <img src={images[0].thumb.clone()} alt="Embed Content" />
                    }
                } else {
                    html! {}
                }
            }
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(PostCardFooter)]
pub fn post_card_footer(props: &PostCardProps) -> Html {
    // create URL for the post
    let slash_split = props
        .feed
        .post
        .uri
        .split('/')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let last_segment = slash_split.last().unwrap();
    let post_url = format!(
        "https://bsky.app/profile/{}/post/{}",
        props.feed.post.author.handle, last_segment
    );
    html! {
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
            <div class="flex items-center">
                <a href={post_url} target="_blank">
                    <i class="fa-solid fa-arrow-up-right-from-square"></i>
                </a>
            </div>
        </div>
    }
}
