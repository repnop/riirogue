use helpers::Coords;

pub trait Creature {
    fn pos(&self) -> Coords;
    fn tile_name(&self) -> &'static str;
    fn take_damage(&mut self, dmg: i32);
    fn deal_damage(&self, c: &mut Creature);
    fn is_dead(&self) -> bool;
    fn hp(&self) -> i32;
}

pub struct Player {
    pub pos: Coords,
    pub inv: Vec<Box<Item>>,
    pub hp: i32,
}

impl Player {
    pub fn new(pos: Coords) -> Player {
        Player {
            pos,
            inv: Vec::new(),
            hp: 30,
        }
    }
}

impl Creature for Player {
    fn pos(&self) -> Coords {
        self.pos
    }

    fn tile_name(&self) -> &'static str {
        "P"
    }

    fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    fn deal_damage(&self, c: &mut Creature) {
        c.take_damage(10);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn hp(&self) -> i32 {
        self.hp
    }
}

pub struct Goblin {
    pub pos: Coords,
    pub atk: i32,
    pub hp: i32,
}

impl Goblin {
    pub fn new(pos: Coords) -> Goblin {
        Goblin {
            pos,
            atk: 5,
            hp: 30,
        }
    }
}

impl Creature for Goblin {
    fn pos(&self) -> Coords {
        self.pos
    }

    fn tile_name(&self) -> &'static str {
        "G"
    }

    fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    fn deal_damage(&self, c: &mut Creature) {
        c.take_damage(self.atk);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn hp(&self) -> i32 {
        self.hp
    }
}

pub trait Item {
    fn id(&self) -> u32;
    fn tile_name(&self) -> &'static str;
    fn consume(&mut self, creature: &mut Creature);
}

pub struct HealingPotion;

impl Item for HealingPotion {
    fn id(&self) -> u32 {
        0
    }

    fn tile_name(&self) -> &'static str {
        "potion"
    }

    fn consume(&mut self, creature: &mut Creature) {
        creature.take_damage(-10);
    }
}
