#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> Comparison {
    use Comparison::*;

    match (l1.len(), l2.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (len_l1, len_l2) if len_l1 > len_l2 => {
            if l1.windows(len_l2).any(|l1_sublist| l1_sublist == l2) {Superlist} else {Unequal}
        }
        (len_l1, len_l2) if len_l1 < len_l2 => {
            if l2.windows(len_l1).any(|l2_sublist| l2_sublist == l1) {Sublist} else {Unequal}
        }
        (_, _) => if l1 == l2 {Equal} else {Unequal}
    }
}
