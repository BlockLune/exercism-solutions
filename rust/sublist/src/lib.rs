#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// 注意这里的子列表和超列表的定义
// [1, 2, 3] 是 [1, 2, 3, 4, 5] 的子列表
// [1, 2, 4] 与 [1, 2, 3, 4, 5] 的关系则是 Unequal

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|sublist_in_second_list| sublist_in_second_list == first_list);
    let superList = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|sublist_in_first_list| sublist_in_first_list == second_list);
    match (sublist, superList) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
