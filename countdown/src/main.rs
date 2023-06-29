trait Operation {
    fn apply(&self) -> Option<i32>;
}

impl Operation for i32 {
    fn apply(&self) -> Option<i32> {
        Some(*self)
    }
}

#[derive(Debug)]
struct Plus<T: Operation, U: Operation>(T, U);

#[derive(Debug)]
struct Minus<T: Operation, U: Operation>(T, U);

#[derive(Debug)]
struct Divide<T: Operation, U: Operation>(T, U);

#[derive(Debug)]
struct Multiply<T: Operation, U: Operation>(T, U);

impl<T: Operation, U: Operation> Operation for Plus<T, U> {
    fn apply(&self) -> Option<i32> {
        if let (Some(left), Some(right)) = (self.0.apply(), self.1.apply()) {
            Some(left + right)
        } else {
            None
        }
    }
}

impl<T: Operation, U: Operation> Operation for Minus<T, U> {
    fn apply(&self) -> Option<i32> {
        if let (Some(left), Some(right)) = (self.0.apply(), self.1.apply()) {
            Some(left - right)
        } else {
            None
        }
    }
}
impl<T: Operation, U: Operation> Operation for Divide<T, U> {
    fn apply(&self) -> Option<i32> {
        if let (Some(left), Some(right)) = (self.0.apply(), self.1.apply()) {
            let div = left / right;
            if left as f64 / right as f64 == div as f64 {
                Some(div)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T: Operation, U: Operation> Operation for Multiply<T, U> {
    fn apply(&self) -> Option<i32> {
        if let (Some(left), Some(right)) = (self.0.apply(), self.1.apply()) {
            Some(left * right)
        } else {
            None
        }
    }
}

fn main() {
    println!("{}", 24.apply().unwrap_or(3));
    println!("{:?}", Plus(3, Plus(2, 4)).apply());
    println!("{:?}", Divide(5, 3).apply());
}
