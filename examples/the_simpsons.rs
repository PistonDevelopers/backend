#[macro_use]
extern crate backend;

pub struct Homer;
pub struct Marge;

#[derive(Eq, PartialEq, Debug)]
pub enum Kids {
    Bart,
    Lisa,
    Maggie,
}

pub enum TheSimpsons {}

pub trait HasParents: OfBackend {
    fn father(&self) -> Self::Father;
    fn mother(&self) -> Self::Mother;
}

pub trait HasChildren: OfBackend {
    fn coolest(&self) -> Self::Kids;
    fn smartest(&self) -> Self::Kids;
    fn cutest(&self) -> Self::Kids;
}

// Creates a `Backend` and `OfBackend` trait.
backend!(
    Father [HasChildren],
    Mother [HasChildren],
    Kids [HasParents]
);

impl HasParents for Kids {
    fn father(&self) -> Homer { Homer }
    fn mother(&self) -> Marge { Marge }
}

impl<T> HasChildren for T
    where T: OfBackend<Kids = Kids>,
{
    fn coolest(&self) -> Kids { Kids::Bart }
    fn smartest(&self) -> Kids { Kids::Lisa }
    fn cutest(&self) -> Kids { Kids::Maggie }
}

backend_impl!(
    TheSimpsons {
        Father = Homer,
        Mother = Marge,
        Kids = Kids
    }
);

// `HasParents` inherits `OfBackend` such that it gets the associated types.
fn father<T: HasParents>(v: T) -> T::Father { v.father() }

pub fn main() {
    let homer: Homer = father(Kids::Bart);
    let marge: Marge = Kids::Lisa.mother();
    assert_eq!(homer.coolest(), marge.coolest());
    assert_eq!(homer.cutest(), Kids::Maggie);   
}
