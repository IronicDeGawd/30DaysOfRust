fn main() {
    // :: is used to access the enum variants
    let weather: Weather = Weather::Cloudy;
    // Calling the implemented method for Weather enum
    weather.call();

    let data1 = Data::Quit;
    // Creating an instance of enum with associated data
    let data2 = Data::Date(08, 05, 2025);

    process_data(data1);
    process_data(data2);
}

// Enum (enumerations) are used to store multiple related values in a single data type. It helps create a group of related distinct values
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Thunderstorms,
}

// Methods can be implemented for enums as well, similar to structs
impl Weather {
    fn call(&self) {
        match self {
            Weather::Cloudy => println!("Cloudy Weather"),
            Weather::Rainy => println!("Rainy Weather"),
            Weather::Sunny => println!("Sunny Weather"),
            Weather::Thunderstorms => println!("Thunderstorms Weather"),
        }
    }
}

// Enums variants can also contain associated data of different data types
enum Data {
    Quit,
    Name { fname: String, lname: String },
    Data(String),
    Flag(bool),
    Date(u8, u8, u16),
}

fn process_data(data: Data) {
    match data {
        Data::Quit => {
            println!("User chose quit")
        }
        Data::Name { fname, lname } => {
            println!("First name: {fname}, Last name: {lname}")
        }
        Data::Data(data) => {
            println!("Data: {data}")
        }
        Data::Flag(flag) => {
            println!("User chose {flag} flag")
        }
        Data::Date(d, m, y) => {
            println!("Data: {d}/{m}/{y}")
        }
    }
}
