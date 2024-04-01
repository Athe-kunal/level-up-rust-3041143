fn median(mut a: Vec<f32>) -> Option<f32> {
    // todo!();
    let vec_len = a.len();

    a.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if a.is_empty(){
        return None;
    }
    else if vec_len%2 == 0{
        let middle_elem_idx = vec_len/2 -1;
        return Some((a[middle_elem_idx] + a[middle_elem_idx+1])/2.0);
    }
    else {
        let middle_elem_idx: usize = (vec_len+1)/2;
        return Some(a[middle_elem_idx-1]);
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
