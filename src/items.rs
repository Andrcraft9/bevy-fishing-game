/// Item System

pub trait Value {
    fn name(&self) -> String;
    fn value(&self) -> f32;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Fish(Fish),
}

impl Value for Item {
    fn name(&self) -> String {
        match self {
            Item::Fish(fish) => fish.name(),
        }
    }

    fn value(&self) -> f32 {
        match self {
            Item::Fish(fish) => fish.value(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FishType {
    Silver,
    Golden,
}

impl Value for FishType {
    fn name(&self) -> String {
        match self {
            FishType::Silver => String::from("Silver"),
            FishType::Golden => String::from("Golden"),
        }
    }

    fn value(&self) -> f32 {
        match self {
            FishType::Silver => 1.0,
            FishType::Golden => 100.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fish {
    pub t: FishType,
    pub weight: f32,
}

impl Value for Fish {
    fn name(&self) -> String {
        match self.t {
            FishType::Silver => format!("Fish - {}", self.t.name()),
            FishType::Golden => format!("Fish - {}", self.t.name()),
        }
    }

    fn value(&self) -> f32 {
        match self.t {
            FishType::Silver => self.weight * self.t.value(),
            FishType::Golden => self.weight * self.t.value(),
        }
    }
}
