fn main() {
  let input = include_str!("./input.txt");

  let mut list1 = Vec::new();
  let mut list2 = Vec::new();

  for line in input.lines() {
    // Split each line into two parts
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        // Parse the parts as integers and add them to the respective arrays
        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            list1.push(left);
            list2.push(right);
        }
    }
  }

  assert_eq!(list1.len(), list2.len(), "Vectors must have the same lenth");

  let mut total_difference = 0;

  list1.sort();
  list2.sort();

  for i in 0..list1.len() {
    let difference = (list1[i] - list2[i]).abs(); // abs is absolute difference
    total_difference += difference;
  }

  println!("Total accumulated difference: {}", total_difference);

  //let output = part1(input);
}

// fn part1(input: &str) -> String {
//   todo!()
// }