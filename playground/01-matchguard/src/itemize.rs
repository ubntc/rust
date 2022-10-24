use super::{Thing,Typ,CarryRule};

const MAX_CARRY_SIZE: usize= 30;
const MAX_CARRY_UNKNOWN_SIZE: usize = 3;
const MAX_CARRY_DOG_SIZE: usize = 5;

impl Typ {
    pub fn is_carriable(&self) -> bool {
        match self {
            Self::Cake => false,
            _ => true,
        }
    }
    pub fn carry_rule(&self) -> CarryRule {
        match self {
            Self::Bird => CarryRule::UseCage,
            _ => CarryRule::UseBox,
        }
    }
}

impl Thing {
    fn has_allowed_carry_size(&self) -> bool {
        if self.size > MAX_CARRY_SIZE {
            return false
        }
        match self.typ {
            Typ::Dog => self.size <= MAX_CARRY_DOG_SIZE,
            Typ::Unknown => self.size <= MAX_CARRY_UNKNOWN_SIZE,
            _ => true
        }
    }

    pub fn is_carriable(&self) -> bool {
        self.typ.is_carriable() && self.has_allowed_carry_size()
    }

    pub fn carry_rule (&self) -> CarryRule {
        if !self.is_carriable() {
            return CarryRule::Forbidden
        }
        self.typ.carry_rule()
    }
}

pub fn check_thing(t: Thing) -> CarryRule { t.carry_rule() }
