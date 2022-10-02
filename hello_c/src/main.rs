use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Result,};

fn main() -> Result<()>{
    let addr = "amqp://127.0.0.1:5672/%2f";
    
    async_global_executor::block_on(async{
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        let queue_name = "hello";
        let mut queue_option = QueueDeclareOptions::default();
        queue_option.durable = true;
        let _queue = channel.queue_declare(queue_name, queue_option, FieldTable::default()).await?;
        let mut consumer = channel.basic_consume(queue_name, "hello consumer", BasicConsumeOptions::default(), FieldTable::default()).await?;
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error");
            match std::str::from_utf8(&delivery.data) {
                Ok(message) => println!("receive {}", message),
                Err(_) => println!("receive invalid message"),
            };
        }
        Ok(())
    })
}
