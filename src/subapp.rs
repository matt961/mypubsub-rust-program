mod conn;

use std::{error::Error, time::Duration};

use lapin::{
    message::{DeliveryResult, Delivery},
    options::{BasicConsumeOptions, BasicNackOptions, BasicAckOptions},
    types::{FieldTable, ReplyCode},
};
use tokio::signal::unix::SignalKind;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .init();

    let rt = tokio::runtime::Builder::new_multi_thread()
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

        let ch_recv = conn.create_channel().await?;
        tracing::info!("created channel");

        tracing::info!(state=?conn.status().state());

        let consumer = ch_recv
            .basic_consume(
                "testqueue",
                "dumb",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        async fn _handle(delivery: DeliveryResult) -> Result<Delivery, Box<dyn Error>> {
            tracing::info!("handling delivery");
            let delivery = delivery?.ok_or("no data")?;
            delivery.ack(BasicAckOptions::default()).await;
            Ok(delivery)
        }

        async fn handle_delivery(delivery: DeliveryResult) {
            let res = _handle(delivery).await;
            match res {
                Ok(res) => {
                    let st = String::from_utf8(res.data).unwrap_or("".to_owned());
                    tracing::info!(data=?st);
                },
                Err(e) => tracing::error!(e),
            };
        }

        consumer.set_delegate(handle_delivery);

        tracing::info!(consumer_state=?consumer.state());

        tokio::signal::unix::signal(SignalKind::terminate())?.recv().await;
        ch_recv.close(ReplyCode::default(), "bye").await?;
        conn.close(ReplyCode::default(), "bye").await?;
        tracing::info!("closed channel and connection.");
        Ok(())
    })
}
