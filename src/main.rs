use std::{net::SocketAddr, ops::Sub, time::Instant};

use anyhow::Context;
use config::{builder::DefaultState, Config, ConfigBuilder, File, FileFormat};
use email_rs::AppConfig;
use lettre::{
    message::{
        header::{self, ContentType},
        Mailbox,
    },
    transport::smtp::PoolConfig,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tokio::net::TcpStream;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config_path = "config/config.toml";
    let config_without_secrets = ConfigBuilder::<DefaultState>::default()
        .add_source(File::new(config_path, FileFormat::Toml))
        .build()?;
    let secrets_path: &str = &config_without_secrets.get_string("secrets_path")?;
    let app_config: AppConfig = Config::builder()
        .add_source(config_without_secrets)
        .add_source(File::new(secrets_path, FileFormat::Ini))
        .build()?
        .try_deserialize()?;

    let (email_result,) = tokio::join!(send(&app_config));
    email_result?;
    Ok(())
}

#[allow(unused)]
async fn test_ipv6(app_config: &AppConfig) -> Result<(), anyhow::Error> {
    //let ip = app_config.remote.ip.parse::<SocketAddr>()?;
    //let port = app_config.remote.port;

    let socket_addr = &app_config
        .remote
        .socket_addr
        .parse::<SocketAddr>()
        .context(anyhow::anyhow!(
            "unable to parse addr {}",
            &app_config.remote.socket_addr
        ))?; // app_config.remote.socket_addr.parse::<SocketAddr>()?;
    let _stream = TcpStream::connect(socket_addr)
        .await
        .context(anyhow::anyhow!(
            "unable to establish tcp connection at {}",
            &app_config.remote.socket_addr
        ))?;

    Ok(())
}

#[allow(unused)]
async fn send(app_config: &AppConfig) -> Result<(), anyhow::Error> {
    let url_result = Url::parse(&app_config.smtp.url);
    let domain = url_result
        .as_ref()
        .map(|url| url.domain().unwrap_or("<unknown>"))
        .unwrap_or("<unknown>");
    let email_body = format!("Delivered via {}", domain);
    let email = Message::builder()
        .from(app_config.email.from.parse::<Mailbox>()?)
        .to(app_config.email.to.parse::<Mailbox>()?)
        .subject(&app_config.email.subject)
        .header(ContentType::TEXT_PLAIN)
        .header(header::References::from("".to_owned()))
        .header(header::InReplyTo::from("".to_owned()))
        .body(email_body.to_owned())
        .context("error creating email message")?;

    let pool_config = PoolConfig::new();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::from_url(&app_config.smtp.url)?
            .pool_config(pool_config)
            .build();

    let start = Instant::now();
    if !mailer.test_connection().await? {
        return Err(anyhow::anyhow!("Smtp client is not connected"));
    }
    println!(
        "Connection test completed in {}ms",
        Instant::now().sub(start).as_millis()
    );

    let start = Instant::now();
    let response = mailer.send(email).await?;
    if !response.is_positive() {
        return Err(anyhow::anyhow!("Smtp response is not positive"));
    }
    println!(
        "Email sent in {}ms, response={:?}",
        Instant::now().sub(start).as_millis(),
        &response
    );

    Ok(())
}
