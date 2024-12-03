import gleam/int
import gleam/list
import gleam/option
import gleam/regexp
import gleam/result
import gleam/string

pub fn pt_1(input: String) -> Int {
  add_multiplications(input)
}

pub fn pt_2(input: String) -> Int {
  let valid_regexp_pattern =
    "(?:^(.*?)don't\\(\\))|(?:do\\(\\)(.*?)don't\\(\\))|(?:do\\(\\)(.*?)$)"

  let options = regexp.Options(multi_line: True, case_insensitive: False)

  let assert Ok(valid_regexp) =
    regexp.compile(valid_regexp_pattern, with: options)

  regexp.scan(valid_regexp, string.replace(input, "\n", ""))
  |> list.map(fn(match) {
    let regexp.Match(_, res) = match
    res
    |> option.values()
    |> list.map(add_multiplications)
  })
  |> list.flatten()
  |> list.fold(0, int.add)
}

fn add_multiplications(text: String) -> Int {
  let assert Ok(reg) =
    regexp.compile(
      "mul\\((\\d+,\\d+)\\)",
      regexp.Options(case_insensitive: False, multi_line: True),
    )

  regexp.scan(reg, text)
  |> list.map(fn(match) {
    let regexp.Match(_, res) = match
    res
    |> option.values()
    |> list.map(fn(nums) {
      nums
      |> string.split(",")
      |> list.filter_map(int.parse)
      |> list.reduce(int.multiply)
      |> result.unwrap(0)
    })
    |> list.fold(0, int.add)
  })
  |> list.fold(0, int.add)
}
