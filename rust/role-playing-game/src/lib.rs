// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None
        }
        else {
            let mut temp: Player = Player {
                health: 100,
                mana: Some(100),
                level: self.level
            };

            if self.level > 9 {
                return Some(temp)
            }
            else {
                temp.mana = None;
                return Some(temp)
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                }
                else {
                    self.health = 0;
                }
                return 0
            }
            Some(u32) => {
                if self.mana >= Some(mana_cost) {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    return mana_cost * 2;
                }
                else {
                    return 0
                }
            }
        }
    }
}
