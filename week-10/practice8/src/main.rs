struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    
    let emp1 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundar Pichai"),
        age:35
    };

    let emp2 = Employee {
        company:String::from("Microsoft Inc."),
        ceo:String::from("Satya Nadella"),
        age:45
    };

    display(emp1);
    display(emp2);
        
}

fn display(emp:Employee){
    println!("Name is :{} company is {} age is {}",emp.ceo,emp.company,emp.age);
}
