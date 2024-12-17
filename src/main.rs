use std::fs;

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price = self.price * 1.1;
        self
    }

    fn show_route(&self) {
        println!(
            "ROUTE: Origin: {} -> Destination: {}",
            self.origin, self.destination
        );
    }

    fn show_flight_info(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    let mut fs18 = Flight::new("Berlin".to_string(), "Paris".to_string(), 100.0, 220);
    fs18.show_flight_info();
    fs18.change_destination("London".to_string())
        .increase_price();
    fs18.show_route();
    fs18.show_flight_info();

    let fs19 = Flight {
        origin: "Egypt".to_string(),
        destination: "Germany".to_owned(),
        ..fs18
    };
    fs19.show_flight_info();
    fs18.show_flight_info();
}
