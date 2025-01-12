use crate::StackRefMut;

use super::Value;

pub struct InterpretedWordParameters<'a, 'b> {
    values: &'a mut StackRefMut<'b, Value>,
}

impl<'a, 'b> InterpretedWordParameters<'a, 'b> {
    pub fn new(values: &'a mut StackRefMut<'b, Value>) -> InterpretedWordParameters<'a, 'b> {
        InterpretedWordParameters { values }
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[inline(always)]
    pub fn values(&self) -> &StackRefMut<Value> {
        &self.values
    }

    #[inline(always)]
    pub fn values_mut(&mut self) -> &mut StackRefMut<'b, Value> {
        &mut self.values
    }
}

pub struct CompiledWordParameters<'a> {
    values: &'a mut StackRefMut<'a, Value>,
}

impl<'a> CompiledWordParameters<'a> {
    pub fn new(values: &'a mut StackRefMut<'a, Value>) -> CompiledWordParameters<'a> {
        CompiledWordParameters { values }
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[inline(always)]
    pub fn values(&self) -> &StackRefMut<Value> {
        &self.values
    }

    #[inline(always)]
    pub fn values_mut(&mut self) -> &mut StackRefMut<'a, Value> {
        &mut self.values
    }
}