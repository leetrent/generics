trait Vehicle {
    // abstract method
    fn start(&self);

    // default method
    fn stop(&self) {
        println!("Vehicle stopped");
    }
}

struct Car {}

impl Vehicle for Car {
    fn start(&self) {
        println!("Car started.")
    }
}

fn start_stop<T: Vehicle>(vehicle: T) {
    vehicle.start();
    vehicle.stop();
}

fn main() {
    let car = Car{};
    start_stop(car);
}