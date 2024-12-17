// Flight struct definition with basic flight attributes
#[derive(Debug)] // Enables debug printing of the struct
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

// Implementation block for Flight struct methods
impl Flight {
    // Constructor method to create a new Flight instance
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    // Method to change flight destination
    // Returns mutable reference for method chaining
    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    // Method to increase price by 10%
    // Returns mutable reference for method chaining
    fn increase_price(&mut self) -> &mut Self {
        self.price = self.price * 1.1;
        self
    }

    // Display flight route information
    fn show_route(&self) {
        println!(
            "ROUTE: Origin: {} -> Destination: {}",
            self.origin, self.destination
        );
    }

    // Display complete flight information using debug format
    fn show_flight_info(&self) {
        println!("{:?}", self);
    }
}
