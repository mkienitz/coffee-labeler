use chromiumoxide::{cdp::browser_protocol::page::CaptureScreenshotFormat, Browser, BrowserConfig};
use color_eyre::eyre::eyre;
use futures::StreamExt;
use maud::{html, Markup, DOCTYPE};

use crate::error::AppError;

pub async fn render_markup(markup: Markup) -> Result<Vec<u8>, AppError> {
    let builder = BrowserConfig::builder();
    let (mut browser, mut handler) =
        Browser::launch(builder.build().map_err(|e| eyre!(e))?).await?;
    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    let page = browser.new_page("about:blank").await?;
    let content = html! {
        (DOCTYPE)
        head {
            link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet";
        }
        body {
            (markup)
        }
    };
    page.set_content(content.into_string()).await?;
    let screenshot = page
        .find_element("#label")
        .await?
        .screenshot(CaptureScreenshotFormat::Png)
        .await?;
    browser.close().await?;
    handle.await?;
    Ok(screenshot)
}
