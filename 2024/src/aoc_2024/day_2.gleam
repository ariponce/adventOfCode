import gleam/int
import gleam/list
import gleam/string

pub fn parse(filename: String) -> List(List(Int)) {
  filename
  |> string.split(on: "\n")
  |> list.map(fn(line) {
    line
    |> string.split(on: " ")
    |> list.map(int.parse)
    |> list.filter_map(fn(x) { x })
  })
}

pub fn pt_1(input: List(List(Int))) {
  input
  |> list.filter(fn(x) { is_safe(x) })
  |> list.length
}

pub fn pt_2(input: List(List(Int))) {
  input
  |> list.filter(fn(x) { is_safe_with_dampener(x) })
  |> list.length
}

fn is_safe(levels: List(Int)) -> Bool {
  is_increasing(levels) || is_decreasing(levels)
}

fn is_increasing(levels: List(Int)) -> Bool {
  case levels {
    [] -> True
    [_] -> True
    [x, y, ..rest] ->
      x < y && int.absolute_value(x - y) <= 3 && is_increasing([y, ..rest])
  }
}

fn is_decreasing(levels: List(Int)) -> Bool {
  case levels {
    [] -> True
    [_] -> True
    [x, y, ..rest] ->
      x > y && int.absolute_value(x - y) <= 3 && is_decreasing([y, ..rest])
  }
}

fn is_safe_with_dampener(levels: List(Int)) -> Bool {
  let indices = list.range(0, list.length(levels))
  list.any(indices, fn(index) {
    let removed_list =
      list.take(levels, index)
      |> list.append(list.drop(levels, index + 1))
    is_safe(removed_list)
  })
}
