use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::{FieldTable, ReplyCode},
    uri::{AMQPAuthority, AMQPUri},
    Connection, ConnectionProperties,
};

pub async fn connect() -> lapin::Result<Connection> {
    let uri = AMQPUri {
        authority: AMQPAuthority {
            host: "rabbitmq".to_owned(),
            userinfo: lapin::uri::AMQPUserInfo {
                username: "dev".to_owned(),
                password: "letmein".to_owned(),
            },
            ..Default::default()
        },
        vhost: "dev".to_owned(),
        ..Default::default()
    };

    Connection::connect_uri(
        uri,
        ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await
}
