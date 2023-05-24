struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

impl Employee {

    fn print(&self) {
        println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    }

    // Alternatively, you can explicitly specify the type of self:
    // fn print(self: &Employee) {
    //    println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    // }
    
    fn payrise(&mut self, amount: u64) {
        self.salary += amount
    }

    // Alternatively, you can explicitly specify the type of self:
    // fn payrise(self: &mut Employee, amount: u64) {
    //     self.salary += amount
    // }

    fn new(name: String, salary: u64, fulltime: bool) -> Employee {
        Employee {
            name, 
            salary, 
            fulltime
        }
    }
}

fn main() {
    
    let employee1 = Employee::new(String::from("Farhad"), 1000, false);
    // employee1.payrise(100); // Nope, because employee1 is immutable.
    employee1.print();

    let mut employee2 = Employee::new(String::from("Faria"), 2000, false);
    employee2.payrise(100);
    employee2.print();
}
