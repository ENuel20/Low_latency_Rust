use serde::{ser::SerializeStruct, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    a : u64,
    #[serde(skip)]
    b : String,
}

/*impl Serialize for Foo {
    fn serialize<S>(&self, S:Serializer) -> Result<S::Ok, S::Error> 
        where
            S : serde::Serializer
    {
        let mut s = serializer.serialize_struct("Foo", 2);
        s.serialize_field("a", &self.a)?;
        s.serialize_field("b", &self.b)?;
        s.end()
    }

} 
*/
fn main() {
    println!("Hello, world!");
}
