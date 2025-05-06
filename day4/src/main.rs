fn main() {
    // We create an instance of the struct by calling it via structName{} and passing the data defined with its value in the parenthesis.
    let st1: Student = Student {
        roll_no: String::from("csai14"),
        name: String::from("Aditya"),
        dept_id: 220,
    };

    println!(
        "Details of student:
          Name = {}
          RollNo = {}
          Department = {}",
        st1.name, st1.roll_no, st1.dept_id
    );

    let student_data = get_data(st1);

    for data in student_data {
        println!("{data}");
    }

    let st2: Student = Student {
        roll_no: String::from("csiot90"),
        name: String::from("Vasu"),
        dept_id: 220,
    };

    // A tuple struct is similar to a regular struct, but its fields do not have names. Instead, they are accessed by their position, just like the elements of a tuple.

    let tupledata: TupleStudent = TupleStudent("15".to_string(), "40".to_string(), 220);

    println!("\nStudent Details : {:?}", st2);
    println!("\nStudent Details : {:?}", tupledata);

    let empty: Empty = Empty;
    empty.student();
}

// STRUCTS:
// Structs are used to save data of different types in a object like structure.
// It helps group related data together.
// Using #[derive(Debug) with structs, we add debugging traits to it. Like using {:?}] to print the struct

#[derive(Debug)]
struct Student {
    roll_no: String,
    name: String,
    dept_id: u8,
}

// TUPLES:
//To create a tuple, use name of the struct and the types of its fields enclosed in parentheses
#[derive(Debug)]
struct TupleStudent(String, String, u32);

// UNIT STRUCTS
// These are special structs with no field. Methods can be implemented for unit structs as well to create distinct types
struct Empty;

// Method Implementation for Structs
impl Empty {
    // &self is similar to this, which refers to variables of current instance of struct
    fn student(&self) {
        println!("Student Details fetched!");
    }
}

fn get_data(stu: Student) -> [String; 3] {
    let name = stu.name;
    let rollno = stu.roll_no;
    let deptid = stu.dept_id.to_string();
    // Fixed size array creation
    let data: [String; 3] = [name, rollno, deptid];
    data
}
