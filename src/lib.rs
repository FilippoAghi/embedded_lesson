#![no_std]
extern crate alloc;

/*
 * #[no_std] must remain and you are not allowed to use any external library.
 *
 * 1- what does it mean to use #[no_std] in a rust project and why may it be necessary?
 * 2- what does it mean to extern crate alloc; in a rust project and why may it be necessary?
 *
 * 3- define a trait that represents a Mex with the following functions where data and id are 
 *    generic  parameters:
 *    - new(id,length,data): return a new Mex
 *    - id(): given a Mex return its id
 *    - dlc(): given a Mex return its length
 *    - data(): given a Mex return its data
 *
 * 4- define a trait that represent the usage of a generic bus where
 *    - the trait has a generic parameter <P> with the constraint that <P> has to implement 
 *      the Mex trait
 *    - the following functions must exist:
 *          - new: initializes a new bus returning the object that implements the trait
 *          - send: use the object that implement the trait, take a <P> type and send it in the bus.
 *              return a Result with type () if Ok and Err(&str) if some errors occur
 *          - receive: use the object that implement the trait,
 *              return a <P> type populated with the data obtained from the bus
 *
 * 5- Implement the following protocols with the trait above:
 *    - CAN base (id 11-bit)
 *    - Serial
 *  For each implementation define some unit-testing where you define at least 
 *  one function for each function defined in the implementation
 *
 *  IF YOU HAVE AN AMD COMPUTER DELETE THE .cargo DIRECTORY OR CHANGE THE target accordingly
 *
 *
 *  Don't worry if you don't complete.
 */

 //use alloc::boxed::Box;
 use alloc::vec::Vec;
 use alloc::collections::vec_deque::VecDeque;

// defines message trait 
 pub trait Mex<I,D> {
    fn new(id: I, length: usize, data: D) -> Self;
    fn id(&self) -> I;
    fn dlc(&self) -> usize;
    fn data(&self) -> D;
 }

 // defines bus trait
 pub trait Bus<P, I, D> 
    where P: Mex<I,D>{

    fn new() -> Self;
    fn send(&mut self, mex: P) -> Result<(), &str>;  
    fn receive(&mut self) -> Result<P, &str>;
 
 }

 // implements Can 
 struct CanMex {
    id: u16,
    length: usize,
    data: Vec<u8>,
}

impl Mex<u16, Vec<u8>> for CanMex {
    fn new(id: u16, length: usize, data: Vec<u8>) -> Self {
        Self { id, length, data }
    }

    fn id(&self) -> u16 {
        self.id
    }

    fn dlc(&self) -> usize {
        self.length
    }

    fn data(&self) -> Vec<u8> { 
        self.data.clone()           //avoid clone
    }
}

struct CanBus {
    messages: VecDeque<CanMex>
}

impl Bus<CanMex, u16, Vec<u8>> for CanBus {  
    fn new() -> Self {                          // TODO better
        Self { 
            messages: VecDeque::new() 
        }
    }


    fn send(&mut self, mex: CanMex) -> Result<(), &str> {

        if mex.length > 8 {
            return Err("Frame data length can't exceed 8 bytes")  // error example #1
        }

        if mex.id > 2047 {
            return Err("ID exceed max value (11 bit available)")  // error example #2
        }

        self.messages.push_back(mex);  // protocol could send frag data (1^ id, 2^ length, 3^ data), need a trait VecDeque
        Ok(())  

        //need out of memory handling otherwise "push" panics
    }

    fn receive(&mut self) -> Result<CanMex, &str> {
        match self.messages.pop_front() {           // protocol could read frag data (1^ id, 2^ length, 3^ data), need a trait VecDeque
            Some(mex) => Ok(mex),
            None => Err("No messages in CanBus")
        }
    }
}

// implements Serial TODO
struct SerialMex {
    id: u8,
    length: usize,
    data: Vec<u8>,
}

impl Mex<u8, Vec<u8>> for SerialMex {
    fn new(id: u8, length: usize, data: Vec<u8>) -> Self {
        Self { id, length, data }
    }

    fn id(&self) -> u8 {
        self.id
    }

    fn dlc(&self) -> usize {
        self.length
    }

    fn data(&self) -> Vec<u8> {  
        self.data.clone()
    }
}

struct SerialBus {
    messages: VecDeque<SerialMex>
}

impl Bus<SerialMex, u8, Vec<u8>> for SerialBus {  
    fn new() -> Self {                          // TODO better
        Self { 
            messages: VecDeque::new() 
        }
    }


    fn send(&mut self, mex: SerialMex) -> Result<(), &str> {

        if mex.length > 8100 {
            return Err("Frame data length can't exceed 100")  // error example #1
        }

        self.messages.push_back(mex); 
        Ok(())  

        //need out of memory handling otherwise "push" panics
    }

    fn receive(&mut self) -> Result<SerialMex, &str> {
        match self.messages.pop_front() {
            Some(mex) => Ok(mex),
            None => Err("No messages in serial")
        }
    }
}
