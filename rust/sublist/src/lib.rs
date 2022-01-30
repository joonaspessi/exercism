#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let first_length = _first_list.len();
    let second_length = _second_list.len();

    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if _first_list.is_empty() {
        return Comparison::Sublist;
    }

    if _second_list.is_empty() {
        return Comparison::Superlist;
    }

    for window in _second_list.windows(first_length) {
        if _first_list == window {
            return Comparison::Sublist;
        }
    }

    for window in _first_list.windows(second_length) {
        if _second_list == window {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}
