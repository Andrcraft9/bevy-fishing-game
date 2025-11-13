/// Item System

pub trait Weight {
    fn weight(&self) -> f32;
}

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

impl Weight for Item {
    fn weight(&self) -> f32 {
        match self {
            Item::Fish(fish) => fish.weight,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FishType {
    Fish,
    Ray,
    Shark,
}

impl Value for FishType {
    fn name(&self) -> String {
        match self {
            FishType::Fish => String::from("Fish"),
            FishType::Ray => String::from("Ray"),
            FishType::Shark => String::from("Shark"),
        }
    }

    fn value(&self) -> f32 {
        match self {
            FishType::Fish => 1.0,
            FishType::Ray => 2.0,
            FishType::Shark => 4.0,
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
        format!("Fish - {}", self.t.name())
    }

    fn value(&self) -> f32 {
        self.weight * self.t.value()
    }
}
