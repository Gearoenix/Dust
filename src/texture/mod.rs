pub mod pixel;

#[derive(Debug, Clone)]
pub struct Texture<C> {
    width: u16,
    height: u16,
    bitmap: Vec<C>
}
