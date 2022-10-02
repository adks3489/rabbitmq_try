use lapin::{options::*, Connection, ConnectionProperties, Result, types::FieldTable, BasicProperties};

fn main() -> Result<()>{
    let addr = "amqp://127.0.0.1:5672/%2f";
    
    async_global_executor::block_on(async{
        let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        let queue_name = "hello";
        let mut queue_option = QueueDeclareOptions::default();
        queue_option.durable = true;
        let _queue = channel.queue_declare(queue_name, queue_option, FieldTable::default()).await?;

        let payload = b"Hello world!";
        let _confirm = channel.basic_publish("", "hello", BasicPublishOptions::default(), payload, BasicProperties::default()).await?.await?;
        println!("Sent {} bytes", payload.len());
        Ok(())
    })
}
