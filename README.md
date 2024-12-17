# Flight Struct Example

This Rust code demonstrates the use of a `Flight` struct to manage flight information.  It showcases struct definition, method implementation, and usage examples.

## Struct Definition

The `Flight` struct is defined with the following attributes:

* `origin: String`: The origin airport.
* `destination: String`: The destination airport.
* `price: f64`: The flight price.
* `passengers: u32`: The number of passengers.

## Methods
The Flight struct implements the following methods:

* `new(origin: String, destination: String, price: f64, passengers: u32) -> Self`: Constructor to create a new Flight instance.
* `change_destination(&mut self, new_destination: String) -> &mut Self`:Changes the flight destination and returns a mutable reference to self for method chaining.
* `increase_price(&mut self) -> &mut Self`: Increases the flight price by 10% and returns a mutable reference to self for method chaining.
* `show_route(&self)`: Prints the flight route (origin and destination).
* `show_flight_info(&self)`: Prints the complete flight information using the Debug format.

## Contributing

Guidelines for contributing to the project:

1. Fork the repository
2. Create a new branch for your feature
3. Make your changes
4. Submit a pull request
