pub fn strtok<'a,'b>(s: &'a mut &'b str, delimeter:char) -> &'b str {
    if let Some(i) = s.find(delimeter)
    {
        let prefix = &s[..i];
        let suffix = &s[(i+delimeter.len_utf8())..];
        *s = suffix;
        prefix
        
    }
    else{
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        assert_eq!(strtok(&mut x, ' '), "hello");
        assert_eq!(x, "world")
    }
}
