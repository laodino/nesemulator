pub struct CPU{
    register_a :u8,
    register_x :u8,
    register_y :u8,
    status:u8,
    program_counter:u8,
}

impl CPU{
   pub fn new()->Self{
          CPU{
            register_a:0,
            register_x:0,
            register_y:0,
            status:0,
            program_counter:0,
          }
    }

    pub fn interpret(&mut self,program:Vec<u8>){
        todo!("")
    }
}