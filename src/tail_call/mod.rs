use crate::Token;

type RuntimeFn = fn(&mut Runtime);

pub struct Runtime<'a> {
    pub bytecode: &'a [usize],
    pub bytecode_pos: usize,

    pub values: &'a mut [usize],
    pub values_pos: usize,
}

pub fn tail_call(tokens: &[Token]) {
    let mut bytecode = Vec::new();

    for tok in tokens {
        match tok {
            Token::Word(word) => match word.as_str() {
                "+" => bytecode.push(add as usize),
                "-" => bytecode.push(sub as usize),
                "*" => bytecode.push(mul as usize),
                "/" => bytecode.push(div as usize),
                "%" => bytecode.push(rem as usize),
                "exit" => bytecode.push(exit as usize),
                _ => (),
            },
            Token::Number(num) => {
                bytecode.push(load as usize);
                bytecode.push(num.to_bits() as usize);
            },
            _ => (),
        }
    }

    let mut values = vec![0; 128];

    let mut run = Runtime {
        bytecode: &bytecode,
        bytecode_pos: 0,
        values: &mut values,
        values_pos: 0,
    };
    start_instruction(&mut run);
}

pub unsafe fn load(run: &mut Runtime) {
    run.bytecode_pos += 1;
    let num = *run.bytecode.get_unchecked(run.bytecode_pos);
    // println!("load {}", f64::from_bits(num as u64));
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = num;
    run.values_pos += 1;
    next_instruction(run)
}

pub unsafe fn add(run: &mut Runtime) {
    // println!("add");
    let a = f64::from_bits(*run.values.get_unchecked(run.values_pos - 1) as u64);
    let b = f64::from_bits(*run.values.get_unchecked(run.values_pos - 2) as u64);
    run.values_pos -= 1;
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = (a + b).to_bits() as usize;
    next_instruction(run)
}

pub unsafe fn sub(run: &mut Runtime) {
    // println!("sub");
    let a = f64::from_bits(*run.values.get_unchecked(run.values_pos - 1) as u64);
    let b = f64::from_bits(*run.values.get_unchecked(run.values_pos - 2) as u64);
    run.values_pos -= 1;
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = (b - a).to_bits() as usize;
    next_instruction(run)
}

pub unsafe fn mul(run: &mut Runtime) {
    // println!("mul");
    let a = f64::from_bits(*run.values.get_unchecked(run.values_pos - 1) as u64);
    let b = f64::from_bits(*run.values.get_unchecked(run.values_pos - 2) as u64);
    run.values_pos -= 1;
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = (a * b).to_bits() as usize;
    next_instruction(run)
}

pub unsafe fn div(run: &mut Runtime) {
    // println!("div");
    let a = f64::from_bits(*run.values.get_unchecked(run.values_pos - 1) as u64);
    let b = f64::from_bits(*run.values.get_unchecked(run.values_pos - 2) as u64);
    run.values_pos -= 1;
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = (b / a).to_bits() as usize;
    next_instruction(run)
}

pub unsafe fn rem(run: &mut Runtime) {
    // println!("rem");
    let a = f64::from_bits(*run.values.get_unchecked(run.values_pos - 1) as u64);
    let b = f64::from_bits(*run.values.get_unchecked(run.values_pos - 2) as u64);
    run.values_pos -= 1;
    let slot = run.values.get_unchecked_mut(run.values_pos);
    *slot = (b % a).to_bits() as usize;
    next_instruction(run)
}

pub unsafe fn exit(run: &mut Runtime) {}

#[inline(always)]
pub fn start_instruction(run: &mut Runtime) {
    unsafe {
        let instr = *run.bytecode.get_unchecked(run.bytecode_pos);
        let func = unsafe { std::mem::transmute::<usize, RuntimeFn>(instr) };
        func(run)
    }
}

#[inline(always)]
pub unsafe fn next_instruction(run: &mut Runtime) {
    run.bytecode_pos += 1;
    let instr = *run.bytecode.get_unchecked(run.bytecode_pos);
    let func = unsafe { std::mem::transmute::<usize, RuntimeFn>(instr) };
    func(run)
}
