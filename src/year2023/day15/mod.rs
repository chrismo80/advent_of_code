pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").split(',').collect();

    let result1 = input.iter().map(|&x| hash(x)).sum::<usize>();

    let mut boxes = vec![linked_hash_map::LinkedHashMap::<&str, u8>::new(); 256];

    for item in input {
        let mut step = item.split(|c| c == '=' || c == '-');

        let label = step.next().unwrap();
        let focal_length = step.next().unwrap();

        let box_id = hash(label);

        if focal_length.is_empty() {
            boxes[box_id].remove(label);
        }
        else {
            *boxes[box_id].entry(label).or_insert(0) = focal_length.parse().unwrap();
        }
    }

    let mut result2 = 0;

    for (b, _box) in boxes.iter().enumerate() {
        for l in 0.._box.len() {
            let label = _box.keys().nth(l).unwrap();
            result2 += (b + 1) * (l + 1) * boxes[b][label] as usize;
        }
    }

    println!("15\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn hash(text: &str) -> usize
{
    let mut value = 0;
    for c in text.chars() {
        value = (value + (c as usize)) * 17 % 256;
    }
    value
}

#[test]
fn test()
{
    assert_eq!(solve(), (511416, 290779));
}
