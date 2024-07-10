pub mod option;
pub mod result;

pub enum MultiState<Fut, FutFunc, FuncOutput> {
    Waiting { future: Fut, func: FutFunc },
    Processing { future: FuncOutput },
    Done,
}
pub enum State<Fut, FutFunc> {
    Waiting { future: Fut, func: FutFunc },
    Done,
}

pub enum MultiStateX<Fut, Input, FutFunc, FuncOutput> {
    Waiting {
        future: Fut,
        input: Input,
        func: FutFunc,
    },
    Processing {
        future: FuncOutput,
    },
    Done,
}
