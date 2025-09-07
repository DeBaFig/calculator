pub fn add(a: u32, b:u32) -> u32 {
   a + b
}

pub fn sub(a: u32, b:u32) -> u32 {
    if a < b {
         b - a
    } else{
         a - b
    }
}
