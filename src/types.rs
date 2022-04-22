use uuid::Uuid;

pub trait RHomeObject {
    fn id(&self) -> Uuid;
}
