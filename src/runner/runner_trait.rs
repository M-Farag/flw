pub trait RunnerTrait {
    fn new(tasks_file:String, input_file:String) -> Self;
    fn run(&self);
}