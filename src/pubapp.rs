mod conn;

use std::{error::Error, time::Duration};

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::{FieldTable, ReplyCode},
};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        let conn = conn::connect().await?;
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

        for i in 0..10 {
            let _confirmation = ch_send
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
                .await?
                .await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
            tracing::info!("sent message {}", i);
        }

        ch_send.close(ReplyCode::default(), "bye").await?;
        conn.close(ReplyCode::default(), "bye").await?;

        tracing::info!("closed channel and connection.");
        Ok(())
    })
}
