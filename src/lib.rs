#[macro_use]
extern crate helix;

use helix::{Error, FromRuby};

ruby! {
  reopen class Array {
    def sort(&self) -> Result<Vec<T>, Error> {
      return self.sort();
    }
  }
}

// impl Array {
//     fn as_vec(&self) -> Result<Vec<Ord>, Error> {
//         Vec::<Ord>::from_ruby(self.helix).map(Vec::<Ord>::from_checked)
//     }
// }
