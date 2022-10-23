#[derive(PartialEq)]
enum Typ {
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

struct Thing {
    size: usize,
    typ: Typ,
}

#[derive(Debug)]
enum CarryRule {
    Forbidden,
    UseCage,
    UseBox,
}

fn main() {
    println!("unknown:        {:?}", check_thing(Thing{size: 0,   typ: Typ::Unknown}));
    println!("veggy:          {:?}", check_thing(Thing{size: 1,   typ: Typ::Veggy}));
    println!("meat:           {:?}", check_thing(Thing{size: 2,   typ: Typ::Meat}));
    println!("cake:           {:?}", check_thing(Thing{size: 1,   typ: Typ::Cake}));
    println!("bird:           {:?}", check_thing(Thing{size: 2,   typ: Typ::Bird}));
    println!("pig:            {:?}", check_thing(Thing{size: 10,  typ: Typ::Pig}));
    println!("cat:            {:?}", check_thing(Thing{size: 2,   typ: Typ::Cat}));
    println!("dog small:      {:?}", check_thing(Thing{size: 4,   typ: Typ::Dog}));
    println!("dog big:        {:?}", check_thing(Thing{size: 6,   typ: Typ::Dog}));
    println!("elephant big:   {:?}", check_thing(Thing{size: 100, typ: Typ::Elephant}));
    println!("elephant small: {:?}", check_thing(Thing{size: 20,  typ: Typ::Elephant}));
}

fn check_thing(t: Thing) -> CarryRule {
    // In a real app, these vars would be consts or external configuration.
    // They are here to simulate some complexity coming from the outside.
    let forbidden = vec![Typ::Cake];
    let max_carry_size = 30;
    let max_carry_unknown_size = 3;
    let max_carry_dog_size = 5;

    // First apply the general rules and use early returns.
    // This is more readable then if-else-ing all cases from the beginning.
    if t.size > max_carry_size {
        return CarryRule::Forbidden
    }
    if forbidden.contains(&t.typ) {
        return CarryRule::Forbidden
    }

    // Match all cases that need special handling using a single `enum`.
    // The rust compiler will help you to have a match for any enum value.
    // Implement special case logic as match guards for the matching enum value.
    match t.typ {
        Typ::Dog if t.size > max_carry_dog_size => CarryRule::Forbidden,
        Typ::Unknown if t.size > max_carry_unknown_size => CarryRule::Forbidden,
        Typ::Bird => CarryRule::UseCage,
        // use a default for all non-special cases
        _ => CarryRule::UseBox,
    }
}
