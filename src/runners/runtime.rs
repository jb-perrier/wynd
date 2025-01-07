use super::Value;

pub struct RuntimeImpl<'a> {
    pub values: &'a mut [Value],
    // always points to the next free value
    pub next_value: usize,

    pub frames: &'a mut [usize],
    // always points to the current frame
    pub current_frame: usize,
}

impl Runtime for RuntimeImpl<'_> {
    #[inline(always)]
    fn values(&mut self) -> &mut [Value] {
        self.values
    }

    #[inline(always)]
    fn frames(&mut self) -> &mut [usize] {
        self.frames
    }

    #[inline(always)]
    fn next_value(&mut self) -> usize {
        self.next_value
    }

    #[inline(always)]
    fn current_frame(&mut self) -> usize {
        self.current_frame
    }

    #[inline(always)]
    fn push_frame(&mut self) {
        self.frames[self.current_frame] = self.next_value;
        self.current_frame += 1;
    }

    #[inline(always)]
    fn pop_frame(&mut self) {
        self.current_frame -= 1;
    }

    #[inline(always)]
    fn push_value(&mut self, value: Value) {
        self.values[self.next_value] = value;
        self.next_value += 1;
    }

    #[inline(always)]
    fn pop_value(&mut self) {
        self.next_value -= 1;
    }

    #[inline(always)]
    fn peek_value(&mut self, index_from_top: usize) -> &Value {
        &self.values[self.next_value - index_from_top - 1]
    }
}

pub trait Runtime {
    fn values(&mut self) -> &mut [Value];
    fn push_value(&mut self, value: Value);
    fn next_value(&mut self) -> usize;
    fn pop_value(&mut self);
    fn peek_value(&mut self, index_from_top: usize) -> &Value;

    fn frames(&mut self) -> &mut [usize];
    fn current_frame(&mut self) -> usize;
    fn push_frame(&mut self);
    fn pop_frame(&mut self);
}