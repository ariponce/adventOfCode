import gleam/int
import gleam/list
import gleam/order
import gleam/result
import gleam/string

pub fn pt_1(input: String) -> Int {
  let #(updates, rules) = get_updates_and_rules(input)
  updates
  |> list.fold(0, fn(acc, update) {
    case updates_are_valid(update, rules) {
      True -> get_middle_element(update)
      False -> "invalid"
    }
    |> int.parse()
    |> result.unwrap(0)
    |> int.add(acc)
  })
}

pub fn pt_2(input: String) {
  let #(updates, rules) = get_updates_and_rules(input)
  updates
  |> list.fold(0, fn(acc, update) {
    case updates_are_valid(update, rules) {
      True -> ""
      False -> sort_invalid(update, rules)
    }
    |> int.parse()
    |> result.unwrap(0)
    |> int.add(acc)
  })
}

fn get_updates_and_rules(input: String) {
  let assert [rules, updates] =
    input
    |> string.split("\n\n")

  let rules =
    rules
    |> string.split("\n")

  let updates =
    updates
    |> string.split("\n")
    |> list.map(fn(update) { string.split(update, ",") })
    |> list.filter(fn(update) { update != [] })

  #(updates, rules)
}

fn get_applicable_rules(update, rules) {
  list.fold(rules, [], fn(acc, cur) {
    case string.contains(cur, update) {
      True -> list.append(acc, [cur])
      False -> acc
    }
  })
}

fn sort_invalid(update, rules) {
  list.sort(update, fn(a, b) {
    let relevant_rule =
      list.find(rules, fn(rule) {
        string.contains(rule, a) && string.contains(rule, b)
      })
      |> result.unwrap("")

    let joined = string.join([a, b], "|")
    case joined == relevant_rule {
      True -> order.Lt
      False -> order.Gt
    }
  })
  |> get_middle_element()
}

fn is_valid_update(target, rest, rules) {
  case rest {
    [first, ..rest] -> {
      let rule =
        list.find(rules, fn(rule) {
          string.contains(rule, target) && string.contains(rule, first)
        })
      case rule {
        Ok(rule) -> {
          let joined = string.join([target, first], "|")
          case joined == rule {
            True -> is_valid_update(target, rest, rules)
            False -> False
          }
        }
        Error(_) -> is_valid_update(target, rest, rules)
      }
    }
    [] -> True
  }
}

fn updates_are_valid(updates, rules) {
  case updates {
    [first, ..rest] -> {
      let applicable_rules = get_applicable_rules(first, rules)
      case is_valid_update(first, rest, applicable_rules) {
        True -> updates_are_valid(rest, rules)
        False -> False
      }
    }
    [] -> True
  }
}

fn get_middle_element(lst: List(String)) -> String {
  case list.drop(lst, list.length(lst) / 2) {
    [] -> ""
    [middle, ..] -> middle
  }
}
