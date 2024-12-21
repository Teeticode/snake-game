
#[derive(Debug)]
enum PersonId {
    Passport(String),
    IdentityCard(String),
}
struct Person {
    name:String,
    age: u32,
    city: String,
    id: PersonId
}

impl Person {
    // associated function
    fn new()-> Person{
        Person{
            name: "Default".to_string(),
            age: 0,
            city: "Default".to_string(),
            id:PersonId::IdentityCard("DEK3uj548".to_string())
        }
    }

    fn from(name:String, age:u32, city:String, id:PersonId) -> Person{
        Person{
            name,
            age,
            city,
            id
        }
    }

    fn change_age( &mut self, new_age:u32){
        self.age = new_age;
       
    }

    fn display_info(&self) {
        println!("{} {} {} {:?}", self.name, self.age, self.city, self.id);
    }
}

fn main (){
    //Person::some_func();
    let mut person = Person::from("Dennis".to_string(), 56, "Nairobi".to_string(), PersonId::Passport("DEK389749".to_string()));

    person.change_age(20000);
    person.display_info();

    

    println!("{} {} {} {:?}", person.age, person.name, person.city, person.id);
}

fn print_message (mut a: String) -> String {    
    a.push_str(" This is cool");
    a
}

