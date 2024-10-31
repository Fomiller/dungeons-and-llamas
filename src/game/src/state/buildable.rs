use std::any::Any;

pub trait SortKeyBuildable: Any {
    fn build(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
