pub trait Named {
    fn name(&self) -> &String;
}
pub trait Report {
    fn report(&self) -> String;
}

pub trait Device: Named + Report {}
