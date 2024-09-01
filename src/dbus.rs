use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use zbus::proxy;
use zbus::zvariant::Structure;
use zbus::{
    zvariant::{OwnedObjectPath, Value},
    Connection,
};

#[proxy(
    interface = "org.freedesktop.portal.Screenshot",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait Screenshot {
    fn pick_color(
        &self,
        parent_window: &str,
        options: HashMap<&str, &Value<'_>>,
    ) -> zbus::Result<OwnedObjectPath>;
}

// not sure if this is correct
#[proxy(
    interface = "org.freedesktop.portal.Request",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Request {
    #[zbus(signal)]
    fn response(&self, response: u32, results: HashMap<&str, Value<'_>>) -> Result<()>;
}

pub async fn grab_color() -> Result<Vec<f32>, Box<dyn Error>> {
    let connection = Connection::session().await?;

    // call the screenshot portal
    let ss_proxy = ScreenshotProxy::new(&connection).await?;
    let path = ss_proxy.pick_color("", HashMap::new()).await?;

    // fetch object from request
    let req_proxy = RequestProxy::new(&connection, path).await?;
    if let Some(msg) = req_proxy.receive_response().await?.next().await {
        let response = msg.args()?;
        if let Some(value) = response.results.get("color") {
            let vec = value
                .downcast_ref::<Structure>()
                .unwrap()
                .fields()
                .iter()
                .map(|it| it.downcast_ref::<f64>().unwrap())
                .map(|it| it as f32)
                .collect::<Vec<f32>>();
            return Ok(vec);
        }
    }
    // fixme
    return Ok(vec![0_f32, 0_f32, 0_f32]);
}
