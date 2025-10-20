use chrono::{Duration, NaiveTime};

use crate::consts;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityType {
    Grass,
    Bush,
    Carrot,
    Pumpkin,
    Tree,
}

impl EntityType {
    pub fn growth_time(&self) -> Duration {
        match self {
            EntityType::Grass => {
                Duration::milliseconds((consts::GRASS_GROWTH_TIME * 1000.0) as i64)
            }
            EntityType::Bush => Duration::milliseconds((consts::BUSH_GROWTH_TIME * 1000.0) as i64),
            EntityType::Carrot => {
                Duration::milliseconds((consts::CARROT_GROWTH_TIME * 1000.0) as i64)
            }
            EntityType::Pumpkin => {
                Duration::milliseconds((consts::PUMPKIN_GROWTH_TIME * 1000.0) as i64)
            }
            EntityType::Tree => Duration::milliseconds((consts::TREE_GROWTH_TIME * 1000.0) as i64),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entity {
    growth_time: NaiveTime,
    entity_type: EntityType,
}

impl Entity {
    pub fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }
    pub fn set_entity_type(&mut self, entity_type: EntityType) -> &mut Self {
        self.entity_type = entity_type;
        self
    }
    pub fn growth_time(&self) -> NaiveTime {
        self.growth_time
    }
    pub fn is_grown(&self) -> bool {
        chrono::Local::now().time() >= self.growth_time
    }
    #[cfg(test)]
    pub fn is_grown_at(&self, time: NaiveTime) -> bool {
        time >= self.growth_time
    }
}

impl From<EntityType> for Entity {
    fn from(entity_type: EntityType) -> Self {
        let now = chrono::Local::now().time();
        let growth_duration = entity_type.growth_time();
        let growth_time = now + growth_duration;

        Entity {
            growth_time,
            entity_type,
        }
    }
}
