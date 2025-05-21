Define a trait called Printable with a method print_info(&self).
Then, implement this trait for two structs: Book and Movie.

- Book has fields: title: String, author: String

- Movie has fields: title: String, director: String

Now, write a generic function display_info<T: Printable>(item: T) that accepts any type that implements the Printable trait and calls its print_info() method.
