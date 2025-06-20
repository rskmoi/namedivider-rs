use crate::divider::divided_name::DividedName;

pub trait NameDivider {
    fn divide_name(&self, undivided_name: &String) -> DividedName;
}
