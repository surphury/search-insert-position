mod test;

use std::convert::TryInto;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let contains_target = nums.contains(&target);

    if !contains_target {
        let defered_index = nums.iter().position(|number| number > &target);

        match defered_index {
            Some(index) => index.try_into().unwrap(),
            None => nums.len().try_into().unwrap(),
        }
    } else {
        nums.iter()
            .position(|number| number == &target)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

fn main() {}
