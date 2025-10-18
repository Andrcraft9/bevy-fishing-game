/// Type Definitions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    Rectangle,
    Circle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Primitive(PrimitiveType),
}
