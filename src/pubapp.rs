mod conn;

use std::{error::Error, time::Duration, borrow::Borrow};

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::{FieldTable, ReplyCode},
};
use tokio::signal::unix::SignalKind;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .init();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let host = std::env::var("AMQP_HOST").unwrap_or("rabbitmq".to_owned());
    let vhost = std::env::var("AMQP_VHOST").unwrap_or("dev".to_owned());
    let username = std::env::var("AMQP_USERNAME").unwrap_or("dev".to_owned());
    let password = std::env::var("AMQP_PASSWORD").unwrap_or("letmein".to_owned());
    let amqp_conf = conn::AMQPConnConf {
        host: host.as_str(),
        vhost: vhost.as_str(),
        username: username.as_str(),
        password: password.as_str(),
    };

    rt.block_on(async {
        let conn = conn::connect(&amqp_conf).await?;
        tracing::info!("created connection");

        let ch_send = conn.create_channel().await?;
        tracing::info!("created channel");

        tracing::info!(state=?conn.status().state());

        let testqueue = ch_send
            .queue_declare(
                "testqueue",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        tracing::info!(consumer_count=?testqueue.consumer_count());

        let stopctl_ch = tokio::sync::watch::channel(false);
        let j = tokio::spawn(async move {
            loop {
                if *stopctl_ch.1.borrow() {
                    ch_send.close(ReplyCode::default(), "bye").await;
                    conn.close(ReplyCode::default(), "bye").await;
                    tracing::info!("closed channel and connection.");
                    break;
                }

                let maybe_pubcon = ch_send
                    .basic_publish(
                        "",
                        "testqueue",
                        BasicPublishOptions::default(),
                        r#"
                {
                    "key": "aaaaa",
                    "value: "bbbbbb"
                }
                "#
                .as_bytes(),
                AMQPProperties::default()
                .with_content_type("application/json".into())
                .with_content_encoding("utf-8".into()),
                    )
                    .await;
                tokio::time::sleep(Duration::from_millis(256)).await;
                tracing::info!("sent message");
            }
        });

        let mut sighdl = tokio::signal::unix::signal(SignalKind::terminate())?;

        sighdl.recv().await;

        stopctl_ch.0.send(true)?;
        j.await?;

        Ok(())
    })
}
