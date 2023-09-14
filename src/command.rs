lazy_static!{
    pub static ref CNXN:Command = Command::new('C','N','X','N');
}

pub struct Command{
    pub command: u32,
    pub magic: u32
}

impl Command{
    pub fn new(a:char,b:char,c:char,d:char)->Command{
        let cmd:u32=((a as u32)<<24) | ((b as u32) <<16) | ((c as u32)<<8) | (d as u32);
        Command{
            command:cmd,
            magic:cmd ^ 0xffffffff
        }
    }
}