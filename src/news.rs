use std::fmt;
use std::str::Chars;

use crate::utils::request_bytes;
use crate::Error;
use htmlescape::decode_html;
use reqwest::Client;
use rss::Channel;
use url::Url;

/// A news item that it returns when iterating over `Content`.
///
/// While content implements display and you can just print it. You may want to handle some of the
/// tags differently, for example adding color to the code blocks.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum NewsItem {
    /// A code block.
    Code(String),
    /// Test.
    Text(String),
    /// A URL.
    Url(String, String),
}

/// The Cnontent of a news entry.
///
/// This implements Display so can simply be printed. Or you can iterate over the tags manually to
/// manually format some things.
#[derive(Debug, Clone)]
pub struct Content<'a> {
    chars: Chars<'a>,
}

impl<'a> fmt::Display for Content<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.clone() {
            match item {
                NewsItem::Text(s) => s.fmt(f)?,
                NewsItem::Code(s) => s.fmt(f)?,
                NewsItem::Url(u, s) => write!(f, "{} ({})", s, u)?,
            }
        }

        Ok(())
    }
}

impl<'a> Iterator for Content<'a> {
    type Item = NewsItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.as_str().is_empty() {
            None
        } else if self.chars.as_str().starts_with("<code>") {
            self.chars.by_ref().take(6).count();
            let code = self.chars.as_str().splitn(2, '<').next().unwrap();
            let code = decode_html(code).unwrap_or_else(|_| code.to_string());
            self.chars.by_ref().any(|c| c == '>');
            Some(NewsItem::Code(code))
        } else if self.chars.as_str().starts_with("</p>") {
            self.chars.by_ref().take(4).count();
            Some(NewsItem::Text("\n".to_string()))
        } else if self.chars.as_str().starts_with("<a") {
            let split = self.chars.as_str().splitn(2, "href=\"");
            let link = split.last().unwrap();
            let link = link.splitn(2, '"').next().unwrap();
            let link = decode_html(link).unwrap_or_else(|_| link.to_string());
            self.chars.by_ref().any(|c| c == '>');
            let word = self.chars.as_str().splitn(2, '<').next().unwrap();
            let word = decode_html(word).unwrap_or_else(|_| word.to_string());
            self.chars.by_ref().any(|c| c == '>');
            Some(NewsItem::Url(link, word))
        } else if self.chars.as_str().starts_with('<') {
            self.chars.by_ref().any(|c| c == '>');
            self.next()
        } else {
            let mut split = self.chars.as_str().splitn(2, '<');
            let text = split.next().unwrap();
            self.chars.by_ref().take(text.chars().count()).count();
            let text = decode_html(text).unwrap_or_else(|_| text.to_string());
            Some(NewsItem::Text(text))
        }
    }
}

/// A news entry.
#[derive(Debug, Clone)]
pub struct News {
    rss: rss::Item,
    content: String,
}

impl<'a> fmt::Display for News {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.date().unwrap_or("No Title"))?;
        f.write_str(" -- ")?;
        f.write_str(self.title().unwrap_or("No Date"))?;
        f.write_str("\n\n")?;
        self.content().fmt(f)?;
        Ok(())
    }
}

impl News {
    /// Gets the content of the news entry.
    pub fn content(&self) -> Content {
        Content {
            chars: self.rss.description().unwrap_or_default().chars(),
        }
    }

    /// Gets the title of the news entry.
    pub fn title(&self) -> Option<&str> {
        self.rss.title()
    }

    /// Gets the date the news entry was published,
    pub fn date(&self) -> Option<&str> {
        self.rss.pub_date()
    }
}

/// Fetch archlinux news.
///
/// The Arrchlinux news is given as an rss feed. The Text in that feed is then html, So best effort
/// parsing is done to make is presentable. <p> <a> and <code> are handled. The rest of the tags
/// are ignored.
///
/// url should point to the news feed you with to use.
/// For example "https://archlinux.org/feeds/news".
pub async fn news(client: &Client, url: Url) -> Result<Vec<News>, Error> {
    let bytes = request_bytes(client, url).await?;

    let channel = Channel::read_from(bytes.as_ref())?;
    let mut news = Vec::new();

    for item in channel.into_items().into_iter() {
        let content = decode_html(item.content().unwrap_or_default()).unwrap_or_default();
        let n = News { rss: item, content };
        news.push(n);
    }

    Ok(news)
}
