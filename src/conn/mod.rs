use lapin::{
    uri::{AMQPAuthority, AMQPUri},
    Connection, ConnectionProperties,
};

pub struct AMQPConnConf<'a> {
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub vhost: &'a str
}

pub async fn connect(conf: &AMQPConnConf<'_>) -> lapin::Result<Connection> {
    let uri = AMQPUri {
        authority: AMQPAuthority {
            host: conf.host.to_owned(),
            userinfo: lapin::uri::AMQPUserInfo {
                username: conf.username.to_owned(),
                password: conf.password.to_owned(),
            },
            ..Default::default()
        },
        vhost: conf.vhost.to_owned(),
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
