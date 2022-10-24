use super::{Thing,Typ,CarryRule};

pub fn check_thing(t: Thing) -> CarryRule {
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
