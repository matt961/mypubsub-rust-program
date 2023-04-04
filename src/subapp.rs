mod conn;

use std::{error::Error, time::Duration};

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    protocol::basic::AMQPProperties,
    types::{FieldTable, ReplyCode},
    uri::{AMQPAuthority, AMQPUri},
    Connection, ConnectionProperties,
};

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
