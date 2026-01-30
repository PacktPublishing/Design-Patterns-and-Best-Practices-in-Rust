use samsa::{Broker, Producer, Consumer};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Samsa pub/sub example...");
    
    // Create the central broker
    let broker = Arc::new(Broker::new());
    
    // Create a producer that sends data DOWN to the broker
    let producer = Producer::new(broker.clone());
    
    // Create a consumer that pulls data DOWN from the broker
    let mut consumer = Consumer::from_beginning(broker.clone(), "greetings");
    
    // Producer sends messages (data flows DOWN)
    println!("Sending messages...");
    producer.send_text("greetings", "Hello, Samsa!")?;
    producer.send_text("greetings", "How are you today?")?;
    producer.send_text("greetings", "Goodbye!")?;
    
    // Consumer receives messages (data flows DOWN)
    println!("Receiving messages...");
    while let Some(event) = consumer.poll()? {
        if let Some(text) = event.message.as_text() {
            println!("Received: {} (offset: {})", text, event.offset);
        }
        
        // Break after receiving the goodbye message
        if event.message.as_text() == Some("Goodbye!") {
            break;
        }
    }
    
    println!("Example completed successfully!");
    Ok(())
}