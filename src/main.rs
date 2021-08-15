#![allow(non_snake_case)]

mod team_book;

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

    let mut team_members = team_book::Members::new();
    team_members.update("Alice", "begin");
    team_members.update("Bob", "good morning");

    print!("{}", team_members.get_member_status("Alice"));
    print!("{}", team_members.get_member_status("Bob"));

    team_members
    .update("Alice", "break")
    .update("Bob", "exit(5)");

    print!("{}", team_members.get_member_status("Alice"));
    print!("{}", team_members.get_member_status("Bob"));
}
