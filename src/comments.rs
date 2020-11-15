use crate::utils::request;
use crate::Error;
use kuchiki::traits::TendrilSink;
use reqwest::Client;
use url::Url;

/// An AUR comment.
#[derive(Clone, Debug)]
pub struct Comment {
    /// The title of the comment. This includes the author and date.
    pub title: String,
    /// The content of the comment itself.
    pub content: String,
}

/// Get the comments from an AUR package.
///
/// URL should be the URL for the AUR.
pub async fn get_comments(client: &Client, url: &Url, pkg: &str) -> Result<Vec<Comment>, Error> {
    let url = url.join(&format!("packages/{}/comments?&PP=1000000", pkg))?;
    let text = request(client, url).await?;
    let parser = kuchiki::parse_html();
    let document = parser.one(text);

    let titles = document
        .select("div.comments h4.comment-header")
        .unwrap()
        .map(|node| node.text_contents());

    let comments = document
        .select("div.comments div.article-content")
        .unwrap()
        .map(|node| node.text_contents());

    let iter = titles.zip(comments).collect::<Vec<_>>();
    let mut comments = Vec::with_capacity(iter.len());

    for (title, comment) in iter.into_iter() {
        let comment = Comment {
            title: title.trim().to_string(),
            content: comment.trim().to_string(),
        };
        comments.push(comment);
    }

    Ok(comments)
}
