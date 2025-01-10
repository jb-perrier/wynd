use super::{bytecode, Frame, Value, Words};

pub struct Runtime<'a> {
    values: &'a mut [Value],
    // always points to the next free value
    next_value: usize,

    frames: &'a mut [Frame],
    // always points to the current frame
    next_frame: usize,

    words: &'a mut Words,
}

impl<'a> Runtime<'a> {
    pub fn new(
        values: &'a mut [Value],
        frames: &'a mut [Frame],
        words: &'a mut Words,
    ) -> Runtime<'a> {
        Runtime {
            values,
            next_value: 0,
            frames,
            next_frame: 0,
            words,
        }
    }
    
    pub fn clear(&mut self) {
        self.next_value = 0;
        self.next_frame = 0;
        self.frames[0] = Frame::default();
    }

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
    pub fn frames(&self) -> &[Frame] {
        self.frames
    }

    #[inline(always)]
    pub fn frames_mut(&mut self) -> &mut [Frame] {
        self.frames
    }

    #[inline(always)]
    pub fn push_frame(&mut self, instr_addr: Frame) {
        self.frames[self.next_frame] = instr_addr;
        self.next_frame += 1;
    }

    #[inline(always)]
    pub fn pop_frame(&mut self) {
        self.next_frame -= 1;
    }

    #[inline(always)]
    pub fn frame(&self) -> Option<&Frame> {
        self.frames.get(self.next_frame - 1)
    }

    #[inline(always)]
    pub fn frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.get_mut(self.next_frame - 1)
    }

    #[inline(always)]
    pub fn frame_add(&mut self, incr: usize) -> Option<&Frame> {
        self.frame_mut()?.bytecode += incr;
        self.frame()
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
