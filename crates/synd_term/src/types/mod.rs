use chrono::DateTime;
use schemars::JsonSchema;
use serde::Serialize;
use synd_feed::types::FeedType;

use crate::client::{
    mutation,
    query::{self},
};

mod time;
pub use time::{Time, TimeExt};

mod page_info;
pub use page_info::PageInfo;

#[derive(Debug, Clone)]
pub struct Link {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub title: Option<String>,
}

impl From<query::subscription::Link> for Link {
    fn from(v: query::subscription::Link) -> Self {
        Self {
            href: v.href,
            rel: v.rel,
            media_type: v.media_type,
            title: v.title,
        }
    }
}

impl From<mutation::subscribe_feed::Link> for Link {
    fn from(v: mutation::subscribe_feed::Link) -> Self {
        Self {
            href: v.href,
            rel: v.rel,
            media_type: v.media_type,
            title: v.title,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntryMeta {
    pub title: Option<String>,
    pub published: Option<Time>,
    pub updated: Option<Time>,
    pub summary: Option<String>,
}

impl From<query::subscription::EntryMeta> for EntryMeta {
    fn from(e: query::subscription::EntryMeta) -> Self {
        Self {
            title: e.title,
            published: e.published.map(parse_time),
            updated: e.updated.map(parse_time),
            summary: e.summary,
        }
    }
}

impl From<mutation::subscribe_feed::EntryMeta> for EntryMeta {
    fn from(e: mutation::subscribe_feed::EntryMeta) -> Self {
        Self {
            title: e.title,
            published: e.published.map(parse_time),
            updated: e.updated.map(parse_time),
            summary: e.summary,
        }
    }
}

impl EntryMeta {
    pub fn summary_text(&self, width: usize) -> Option<String> {
        self.summary
            .as_deref()
            .map(|summary| html2text::from_read(summary.as_bytes(), width))
    }
}

#[derive(Debug, Clone)]
pub struct Feed {
    pub r#type: Option<FeedType>,
    pub title: Option<String>,
    pub url: String,
    pub updated: Option<Time>,
    pub links: Vec<Link>,
    pub website_url: Option<String>,
    pub description: Option<String>,
    pub generator: Option<String>,
    pub entries: Vec<EntryMeta>,
    pub authors: Vec<String>,
}

impl From<query::subscription::Feed> for Feed {
    fn from(f: query::subscription::Feed) -> Self {
        Self {
            r#type: match f.type_ {
                query::subscription::FeedType::ATOM => Some(FeedType::Atom),
                query::subscription::FeedType::RSS1 => Some(FeedType::RSS1),
                query::subscription::FeedType::RSS2 => Some(FeedType::RSS2),
                query::subscription::FeedType::RSS0 => Some(FeedType::RSS0),
                query::subscription::FeedType::JSON => Some(FeedType::JSON),
                query::subscription::FeedType::Other(_) => None,
            },
            title: f.title,
            url: f.url,
            updated: f.updated.map(parse_time),
            links: f.links.nodes.into_iter().map(From::from).collect(),
            website_url: f.website_url,
            description: f.description,
            generator: f.generator,
            entries: f.entries.nodes.into_iter().map(From::from).collect(),
            authors: f.authors.nodes,
        }
    }
}

impl From<mutation::subscribe_feed::Feed> for Feed {
    fn from(f: mutation::subscribe_feed::Feed) -> Self {
        Self {
            r#type: match f.type_ {
                mutation::subscribe_feed::FeedType::ATOM => Some(FeedType::Atom),
                mutation::subscribe_feed::FeedType::RSS1 => Some(FeedType::RSS1),
                mutation::subscribe_feed::FeedType::RSS2 => Some(FeedType::RSS2),
                mutation::subscribe_feed::FeedType::RSS0 => Some(FeedType::RSS0),
                mutation::subscribe_feed::FeedType::JSON => Some(FeedType::JSON),
                mutation::subscribe_feed::FeedType::Other(_) => None,
            },
            title: f.title,
            url: f.url,
            updated: f.updated.map(parse_time),
            links: f.links.nodes.into_iter().map(From::from).collect(),
            website_url: f.website_url,
            description: f.description,
            generator: f.generator,
            entries: f.entries.nodes.into_iter().map(From::from).collect(),
            authors: f.authors.nodes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub title: Option<String>,
    pub published: Option<Time>,
    pub updated: Option<Time>,
    pub website_url: Option<String>,
    pub summary: Option<String>,
    pub feed_title: Option<String>,
    pub feed_url: String,
}

impl Entry {
    pub fn summary_text(&self, width: usize) -> Option<String> {
        self.summary
            .as_deref()
            .map(|summary| html2text::from_read(summary.as_bytes(), width))
    }
}

impl From<query::entries::Entry> for Entry {
    fn from(v: query::entries::Entry) -> Self {
        Self {
            title: v.title,
            published: v.published.map(parse_time),
            updated: v.updated.map(parse_time),
            website_url: v.website_url,
            feed_title: v.feed.title,
            feed_url: v.feed.url,
            summary: v.summary,
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct ExportedFeed {
    pub title: Option<String>,
    pub url: String,
}

impl From<query::export_subscription::ExportSubscriptionOutputFeedsNodes> for ExportedFeed {
    fn from(v: query::export_subscription::ExportSubscriptionOutputFeedsNodes) -> Self {
        Self {
            title: v.title,
            url: v.url,
        }
    }
}

fn parse_time(t: impl AsRef<str>) -> Time {
    DateTime::parse_from_rfc3339(t.as_ref())
        .expect("invalid rfc3339 time")
        .with_timezone(&chrono::Utc)
}
