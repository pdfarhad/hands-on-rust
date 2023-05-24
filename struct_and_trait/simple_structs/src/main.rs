struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

fn main() {    
    example_creating_using_struct();
    example_struct_and_functions();
}

fn example_creating_using_struct() {
    
    // Create an immutable structure instance.
    let employee1 = Employee {
        name: String::from("Farhad"),
        fulltime: false,
        salary: 1000
    };
    println!("{} earns {}, fulltime status: {}", employee1.name, employee1.salary, employee1.fulltime);

    // Create a mutable structure instance. Note, Rust doesn't support field-by-field mutability.
    let mut employee2 = Employee {
        name: String::from("Faria"),
        fulltime: false,
        salary: 2000
    };
    employee2.salary *= 2;
    println!("{} earns {}, fulltime status: {}", employee2.name, employee2.salary, employee2.fulltime);
}

fn example_struct_and_functions() {
        
    let employee3 = build_employee(String::from("Nawfaa"), 3000, true);
    print_employee(&employee3);

    let employee4 = build_employee_v2(String::from("Narmeen"), 4000, false);
    print_employee(&employee4);
}

fn build_employee(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {
        name: name,
        salary: salary,
        fulltime: fulltime
    }
}

fn build_employee_v2(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {
        name,
        salary,
        fulltime
    }
}

fn print_employee(emp: &Employee) {
    // Note, we still use . to access fields.
    println!("{} earns {}, fulltime status: {}", emp.name, emp.salary, emp.fulltime);
}