struct Table<M: Model> {
    _boo: M
}

impl<M: Model> Table<M> {
    pub fn create_record(data: M) -> Result<(),()> {
        todo!();
        Ok(())
    }

    pub fn create_records(data: impl Iterator<Item = M>) {
   
    }

    pub fn read_record() -> Result<M, ()>{
        todo!()
    }

    pub fn update_record() -> Result<M, ()>{
        todo!()
    }

    pub fn delete_record() -> Result<M, ()>{
        todo!()
    }
}

trait Model {
    
}

struct User {
    name: String,
    age: i32
}

impl Model for User {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
