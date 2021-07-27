#![allow(non_snake_case)]

enum Niceness
{
    NotNice,
    Nice(String) // detail how nice the person is
}

struct Person
{
    name: String,
    niceness: Niceness
}

impl Person{
    fn little(&self, niceness: Niceness) -> Person
    {
        Person{
            name: format!("Little {}", self.name),
            niceness: niceness
        }
    }
}

fn say_hello(p: &Person)
{
    let greeting = match &p.niceness {
        Niceness::NotNice => String::from("not nice"),
        Niceness::Nice(how) => format!("{} nice", how)
    };
    println!("Hello, {}, {} to meet you.", p.name, greeting);
}

fn main() {
    let bob = Person{
        name: String::from("Bob"),
        niceness: Niceness::Nice(String::from("very")),
    };
    say_hello(&bob);
    let people = [bob];
    let bobby = people[0].little(Niceness::NotNice);
    say_hello(&bobby);
}
