//Traits : It help define shared behavior for multiple types. Traits define a set of methods that a type must implement to satisfy the trai
trait Printable {
    fn print_info(&self) {}

    //default trait : implemented functionality in trait itself
    fn print_release(&self) {
        println!("Release Confirmation Date is weak");
    }
}

trait Watch: Printable {
    fn watch(&self) {}
}

struct Book {
    name: String,
    release_date: u16,
}

struct Movie {
    name: String,
    release_date: u16,
}

// impl keyword is used a to implement common behaviour of different types
impl Printable for Book {
    fn print_info(&self) {
        println!("name: {}, release date : {} ", self.name, self.release_date)
    }
}
impl Printable for Movie {
    fn print_info(&self) {
        println!("name: {}, release date : {} ", self.name, self.release_date)
    }
}

impl Watch for Movie {
    fn watch(&self) {
        println!("Wathcing now!")
    }
}

fn main() {
    let movie = Movie {
        name: String::from("Thunderbolt"),
        release_date: 2025,
    };

    let book = Book {
        name: "Theory of World".to_string(),
        release_date: 2021,
    };

    display_info(&movie);
    display_info(&book);

    movie.watch();
}

//Trait Bound : Only accepts a Generic T of any type if Printable is implemented in ut
fn display_info<T: Printable>(item: &T) {
    item.print_info();
}
