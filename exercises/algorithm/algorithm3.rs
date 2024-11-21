/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// enum Modd {
//     FlagInLeft,
//     FlagInRight
// }

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    let len = array.len();

    // let mut modd = Modd::FlagInLeft;
    let mut modd = true;
    let mut flag = 0;
    let mut compare = len as usize - 1;

    while flag != compare {
        // if (array[flag] > array[compare]) ^ (modd != Modd::FlagInLeft) {
        if (array[flag] > array[compare]) ^ (modd != true) {
            array.swap(flag, compare);
            (flag, compare) = (compare, flag);
            modd = !modd;
        };

        // if modd == Modd::FlagInLeft {
        if modd == true {
            compare -= 1;
        } else {
            compare += 1;
        };
    };

    if flag != 0 {
        sort(&mut array[0..flag]);
    }

    if flag != len - 1 {
        sort(&mut array[flag + 1..len]);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}