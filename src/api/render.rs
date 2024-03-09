use chromiumoxide::{cdp::browser_protocol::page::CaptureScreenshotFormat, Browser, BrowserConfig};
use color_eyre::eyre::eyre;
use futures::StreamExt;
use maud::Markup;

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
    page.set_content(markup.into_string()).await?;
    let screenshot = page
        .find_element("#label")
        .await?
        .screenshot(CaptureScreenshotFormat::Png)
        .await?;
    browser.close().await?;
    handle.await?;
    Ok(screenshot)
}
