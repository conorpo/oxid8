#![allow(unstable_features)]
#![feature(random)]
#![feature(trait_alias)]


//mod file;
// mod btree;
//mod page;
pub mod storage_engine;
// pub mod directory;

#[cfg(test)]
mod tests {
    use super::*;
}
