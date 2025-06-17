use log::{info, warn, error, debug};

struct MyClass {
    name: String,
}

impl MyClass {
    fn new(name: String) -> Self {
        info!("Creating new instance: {}", name);
        Self { name }
    }

    fn process_data(&self, data: &str) -> Result<String, String> {
        debug!("Processing data for {}: {}", self.name, data);
        
        if data.is_empty() {
            warn!("Empty data provided to {}", self.name);
            return Err("Data cannot be empty".to_string());
        }

        let result = format!("Processed: {}", data);
        info!("Successfully processed data for {}", self.name);
        Ok(result)
    }

    fn handle_error(&self, error: &str) {
        error!("Error in {}: {}", self.name, error);
    }
}

fn main() {
    env_logger::init();
    
    info!("Application starting");
    
    let instance = MyClass::new("ExampleClass".to_string());
    
    match instance.process_data("sample data") {
        Ok(result) => info!("Result: {}", result),
        Err(e) => instance.handle_error(&e),
    }
    
    match instance.process_data("") {
        Ok(result) => info!("Result: {}", result),
        Err(e) => instance.handle_error(&e),
    }
    
    info!("Application finished");
}
