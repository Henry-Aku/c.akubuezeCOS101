

fn main() {
    let name = "Aish Lewaal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("University: {} \nAddress: {}",uni,addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "Schoo; of science and Technology";
    println!("Department: {}, \nSchool: {}", department,school); 
}
