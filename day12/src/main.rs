fn main() {
    let input = include_str!("input");
    println!("Part one: {}", solve(input, |s| s.to_owned() + ".", |v| v).unwrap());
    println!(
        "Part two: {}",
        solve(
            input,
            |s| (0..5).map(|_| s).collect::<Vec<_>>().join("?") + ".",
            |v| unfold_sizes(v, 5)
        ).unwrap()
    );
}

fn solve<F1, F2>(input: &str, transform_pattern: F1, transform_sizes: F2) -> Result<usize, &'static str>
where
    F1: Fn(&str) -> String,
    F2: Fn(Vec<usize>) -> Vec<usize>,
{
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let pattern = transform_pattern(parts.get(0).ok_or("Pattern not found")?);
            let sizes = parts.get(1).ok_or("Sizes not found")?
                .split(',')
                .map(|s| s.parse::<usize>().map_err(|_| "Invalid size"))
                .collect::<Result<Vec<_>, _>>()?;
            let transformed_sizes = transform_sizes(sizes);
            Ok((pattern, transformed_sizes))
        })
        .try_fold(0, |acc, line_result| {
            let (pattern, sizes) = line_result?;
            let mut memoization = vec![vec![None; sizes.len()]; pattern.len()];
            Ok(acc + calculate_arrangements(0, 0, &pattern, &sizes, &mut memoization))
        })
}

/// Returns a vector containing `amount` copies of the elements in `vector`.
fn unfold_sizes<T: Clone>(vector: Vec<T>, amount: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(vector.len() * amount);
    for _ in 0..amount {
        result.extend(vector.clone());
    }
    result
}

fn calculate_arrangements(
    index: usize,
    group_index: usize,
    pattern: &str,
    sizes: &Vec<usize>,
    memoization: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // Check if we have processed all groups. If so, verify if the rest of the pattern is valid.
    if group_index == sizes.len() {
        return if pattern[index..].contains('#') {
            0 // Invalid pattern as '#' found after processing all groups.
        } else {
            1 // Valid pattern as no '#' found after processing all groups.
        };
    }

    // Return 0 if we've reached the end of the pattern without processing all groups.
    if index >= pattern.len() {
        return 0;
    }

    // Return the cached result if this state has already been computed.
    if let Some(result) = memoization[index][group_index] {
        return result;
    }

    // Initialize the result for this state.
    let mut result = 0;

    // Get the character at the current index in the pattern.
    let current_char = pattern.chars().nth(index).unwrap();

    // If the current character is not '#', try skipping it and moving to the next character.
    if current_char != '#' {
        result += calculate_arrangements(index + 1, group_index, pattern, sizes, memoization);
    }

    // Check if it's possible to place the current group at this index.
    if index + sizes[group_index] <= pattern.len()
        // Ensure all characters in the group range are not '.'.
        && pattern[index..index + sizes[group_index]].chars().all(|c| c != '.')
        // Ensure the character following the group is not '#'.
        && pattern.chars().nth(index + sizes[group_index]).unwrap_or('.') != '#'
    {
        // If conditions are met, place the group and move to the next group.
        result += calculate_arrangements(
            index + sizes[group_index] + 1,
            group_index + 1,
            pattern,
            sizes,
            memoization,
        );
    }

    // Cache the result for the current state before returning.
    memoization[index][group_index] = Some(result);

    result
}
