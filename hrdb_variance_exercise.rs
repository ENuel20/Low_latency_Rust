pub struct MutRef<'a>{
    s : &mut 'a String
}

impl<'a> MutRef <'a> {
    fn bad(&mut self, f:F)
    where
        F: FnOnce(&mut 'a string){
        f{self.s}
    }

    fn good(&mut self, f:F)
    where
        F: FnOnce(mut 'a string)
    {
        f{self.s}
    }
}

fn main() {

    let s = String::from();
    let mut ms = MutStr { s: &mut s };

    // This triggers the error in modify_bad
     ms.modify_bad(|s| s.push_str(" world"));
    
    // This works
    //ms.modify_good(|s| s.push_str(" world"));
}


