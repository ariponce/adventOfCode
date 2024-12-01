import gleam/int
import gleam/list
import gleam/result
import gleam/string

pub fn parse(filename: String) -> List(List(Int)) {
  filename
  |> string.split(on: "\n")
  |> list.map(fn(line) {
    line
    |> string.split(on: "   ")
    |> list.map(int.parse)
    |> list.filter_map(fn(x) { x })
  })
}

fn extract_lists(input: List(List(Int))) -> #(List(Int), List(Int)) {
  #(
    list.map(input, fn(line) { list.first(line) |> result.unwrap(0) }),
    list.map(input, fn(line) { list.last(line) |> result.unwrap(0) }),
  )
}

pub fn pt_1(input: List(List(Int))) {
  let #(left, right) = extract_lists(input)
  let sorted_left = list.sort(left, int.compare)
  let sorted_right = list.sort(right, int.compare)
  list.zip(sorted_left, sorted_right)
  |> list.map(fn(pair) { int.absolute_value(pair.0 - pair.1) })
  |> int.sum
}

pub fn pt_2(input: List(List(Int))) {
  let #(left, right) = extract_lists(input)
  list.map(left, fn(line) {
    list.filter(right, fn(value) { value == line })
    |> list.length
    |> fn(count) { line * count }
  })
  |> int.sum
}
