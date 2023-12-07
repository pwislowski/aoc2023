require IEx

defmodule Day4.Common do
  def read_data(file_path) do
    case File.read(file_path) do
      {:ok, data} ->
        String.split(data, "\n", trim: true)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end
end

defmodule Day4.Part1 do
  def solve(data) do
    Enum.map(data, fn card ->
      [_, card, winning, scratched] = Regex.run(~r/Card\s+(\d+):(.*)\|(.*)/, card)

      nums =
        Regex.replace(~r/Card.*\d+: /, card, "")
        |> String.split("|")

      winning = parse_numbers(winning)
      scratched = parse_numbers(scratched)

      matches =
        Enum.filter(winning, fn num -> Enum.member?(scratched, num) end)
        |> Enum.count()

      points(matches)
    end)
    |> Enum.sum()
  end

  def parse_numbers(n), do: n |> String.split() |> Enum.map(&String.to_integer/1)

  def points(matches) do
    case matches do
      0 -> 0
      1 -> 1
      _ when matches > 1 -> :math.pow(2, matches - 1)
    end
  end
end

defmodule Day4.Part2 do
  def solve(data) do
    cards =
      data
      |> Enum.map(fn line ->
        [_, card, winning, scratched] = Regex.run(~r/Card\s+(\d+):(.*)\|(.*)/, line)

        %{
          card: String.to_integer(card),
          winning: parse_numbers(winning),
          scratched: parse_numbers(scratched)
        }
      end)

    cards_amount = Map.new(cards, fn %{card: i} -> {i, 1} end)

    cards
    |> Enum.reduce(cards_amount, &process_card/2)
    |> Map.values()
    |> Enum.sum()
  end

  def process_card(card, cards_amount) do
    matches = calculate_matches(card)

    if matches > 0 do
      range = (card.card + 1)..(card.card + matches)
      update_amounts(range, card.card, cards_amount)
    else
      cards_amount
    end
  end

  def update_amounts(range, card_value, cards_amount) do
    card_copies = cards_amount[card_value]

    Enum.reduce(range, cards_amount, fn i, acc ->
      Map.update(acc, i, card_copies, &(&1 + card_copies))
    end)
  end

  def calculate_matches(card),
    do: MapSet.intersection(card.winning, card.scratched) |> MapSet.size()

  def parse_numbers(n), do: n |> String.split() |> Enum.map(&String.to_integer/1) |> MapSet.new()
end

data = Day4.Common.read_data("input_4.txt")
# => 20829
IO.puts("Part 1: #{Day4.Part1.solve(data)}")
# => 12648035
IO.puts("Part 2 : #{Day4.Part2.solve(data)}")
