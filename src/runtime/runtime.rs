use super::{InstructionAddress, Value, Words};

pub struct Runtime<'a> {
    values: &'a mut [Value],
    // always points to the next free value
    next_value: usize,

    frames: &'a mut [InstructionAddress],
    // always points to the current frame
    current_frame: usize,

    words: &'a mut Words,
}

impl Runtime<'_> {
    #[inline(always)]
    pub fn words(&self) -> &Words {
        self.words
    }

    #[inline(always)]
    pub fn words_mut(&mut self) -> &mut Words {
        self.words
    }

    #[inline(always)]
    pub fn values(&self) -> &[Value] {
        self.values
    }

    #[inline(always)]
    pub fn values_mut(&mut self) -> &mut [Value] {
        self.values
    }

    #[inline(always)]
    pub fn frames(&self) -> &[InstructionAddress] {
        self.frames
    }

    #[inline(always)]
    pub fn frames_mut(&mut self) -> &mut [InstructionAddress] {
        self.frames
    }

    #[inline(always)]
    pub fn push_frame(&mut self, instr_addr: InstructionAddress) {
        self.frames[self.current_frame] = instr_addr;
        self.current_frame += 1;
    }

    #[inline(always)]
    pub fn pop_frame(&mut self) {
        self.current_frame -= 1;
    }

    #[inline(always)]
    pub fn frame(&mut self) -> Option<&InstructionAddress> {
        self.frames.get(self.current_frame)
    }

    #[inline(always)]
    pub fn frame_incr_instr(&mut self, incr: usize){
        self.frames[self.current_frame].instr += incr;
    }

    #[inline(always)]
    pub fn push_value(&mut self, value: Value) {
        self.values[self.next_value] = value;
        self.next_value += 1;
    }

    #[inline(always)]
    pub fn pop_value(&mut self) {
        self.next_value -= 1;
    }

    #[inline(always)]
    pub fn peek_value(&mut self, index_from_top: usize) -> &Value {
        &self.values[self.next_value - index_from_top - 1]
    }
}