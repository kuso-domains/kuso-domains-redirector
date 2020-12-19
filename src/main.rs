use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use async_std_resolver as resolver;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .map(|port| port.parse())
        .transpose()
        .unwrap()
        .unwrap_or(8080);
    tide::log::start();
    let resolver = resolver::resolver(
        resolver::config::ResolverConfig::cloudflare(),
        resolver::config::ResolverOpts::default(),
    )
    .await?;
    let resolver = Arc::new(resolver);
    let mut app = tide::new();
    app.at("/").get(move |req: tide::Request<()>| {
        let resolver = resolver.clone();
        async move {
            let host_hdr = req
                .header(tide::http::headers::HOST)
                .ok_or_else(|| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, "No host header")
                })?
                .last()
                .as_str();
            let mut parts = host_hdr.split(':');
            let host = parts.next().unwrap();
            tide::log::info!("host: {}", host);
            let query = format!("_kuso-domains-to.{}", host);
            let txt_lookup = resolver.txt_lookup(query).await?;
            let data = txt_lookup
                .iter()
                .next()
                .and_then(|txt| txt.txt_data().first())
                .ok_or_else(|| {
                    tide::Error::from_str(tide::StatusCode::NotFound, "No TXT records")
                })?;
            let redirect_to = String::from_utf8(data.to_vec())?;
            tide::log::info!("redirect-to: {}", redirect_to);
            let res = tide::Response::builder(tide::StatusCode::SeeOther)
                .header(tide::http::headers::LOCATION, redirect_to)
                .build();
            Ok(res)
        }
    });
    app.listen(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port))
        .await?;
    Ok(())
}
