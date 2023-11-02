pub trait IdGenerator {
    fn generate_id(&self) -> String;
}
