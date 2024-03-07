use chromiumoxide::{cdp::browser_protocol::page::CaptureScreenshotFormat, Browser, BrowserConfig};
use color_eyre::eyre::eyre;
use futures::StreamExt;
use maud::Markup;

use crate::error::AppError;

pub async fn render_markup(markup: Markup) -> Result<Vec<u8>, AppError> {
    let builder = BrowserConfig::builder();
    let (mut browser, mut handler) = Browser::launch(builder.build().map_err(|e| eyre!(e))?)
        .await
        .map_err(|e| eyre!(e))?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser
        .new_page("about:blank")
        .await
        .map_err(|e| eyre!(e))?;

    page.set_content(markup.into_string())
        .await
        .map_err(|e| eyre!(e))?;

    let el = page.find_element("#target").await.map_err(|e| eyre!(e))?;
    let screenshot = el
        .screenshot(CaptureScreenshotFormat::Png)
        .await
        .map_err(|e| eyre!(e))?;
    browser.close().await.map_err(|e| eyre!(e))?;
    handle.await.map_err(|e| eyre!(e))?;
    Ok(screenshot)
}
