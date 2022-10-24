mod itemize;
mod matchguard;
use std::env;

#[derive(PartialEq)]
pub enum Typ {
    Unknown,
    Veggy,
    Meat,
    Cake,
    Bird,
    Pig,
    Cat,
    Dog,
    Elephant,
}

pub struct Thing {
    pub size: usize,
    pub typ: Typ,
}

#[derive(Debug)]
pub enum CarryRule {
    Forbidden,
    UseCage,
    UseBox,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let func = if args.len() > 1 { args[1].as_str() } else { "itemize" };

    println!("using function: {}", func);

    let check = match func {
        "matchguard" => matchguard::check_thing,
        "itemize" => itemize::check_thing,
        _ => panic!("unknown ceck function: {}", func)
    };

    println!("unknown:        {:?}", check(Thing{size: 0,   typ: Typ::Unknown}));
    println!("veggy:          {:?}", check(Thing{size: 1,   typ: Typ::Veggy}));
    println!("meat:           {:?}", check(Thing{size: 2,   typ: Typ::Meat}));
    println!("cake:           {:?}", check(Thing{size: 1,   typ: Typ::Cake}));
    println!("bird:           {:?}", check(Thing{size: 2,   typ: Typ::Bird}));
    println!("pig:            {:?}", check(Thing{size: 10,  typ: Typ::Pig}));
    println!("cat:            {:?}", check(Thing{size: 2,   typ: Typ::Cat}));
    println!("dog small:      {:?}", check(Thing{size: 4,   typ: Typ::Dog}));
    println!("dog big:        {:?}", check(Thing{size: 6,   typ: Typ::Dog}));
    println!("elephant big:   {:?}", check(Thing{size: 100, typ: Typ::Elephant}));
    println!("elephant small: {:?}", check(Thing{size: 20,  typ: Typ::Elephant}));
}