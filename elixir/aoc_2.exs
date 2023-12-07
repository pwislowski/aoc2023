require IEx

defmodule Day2.Common do
  def read_input(file_path) do
    case File.read(file_path) do
      {:ok, data} ->
        String.split(data, "\n", trim: true)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end
end

defmodule Day2.Part1 do
  def solve(data) do
    ids = Enum.map(data, &possible?/1)
    Enum.sum(ids)
  end

  def possible?(line) do
    id =
      Regex.scan(~r/(?<=Game )\d+/, line)
      |> Enum.map(fn [num] -> String.to_integer(num) end)
      |> List.first()

    possible? =
      Regex.replace(~r/Game \d+: /, line, "")
      |> String.split("; ")
      |> Enum.map(&colors_match?/1)
      |> Enum.all?(fn element -> element == true end)

    if possible?, do: id, else: 0
  end

  def colors_match?(game) do
    red = get_color_value("red", game)
    green = get_color_value("green", game)
    blue = get_color_value("blue", game)

    red <= 12 and green <= 13 and blue <= 14
  end

  def get_color_value(color, str) do
    case Regex.run(~r/(\d+)\s#{color}/, str) do
      [_, number] -> String.to_integer(number)
      nil -> 0
    end
  end
end

defmodule Day2.Part2 do
  def solve(data) do
    powers = Enum.map(data, &get_powers/1)
    Enum.sum(powers)
  end

  def get_powers(line) do
    Regex.replace(~r/Game \d+: /, line, "")
    |> get_values()
    |> Enum.reduce(1, fn x, acc -> x * acc end)
  end

  def get_values(game) do
    colors = ["red", "green", "blue"]

    Enum.map(colors, fn color ->
      Regex.scan(~r/(\d+)\s#{color}/, game)
      |> Enum.map(fn [_, num] -> String.to_integer(num) end)
      |> Enum.max()
    end)
  end
end

data = Day2.Common.read_input("input_2.txt")
# => 2285
IO.puts("Part 1: #{Day2.Part1.solve(data)}")
# => 77021
IO.puts("Part 2 : #{Day2.Part2.solve(data)}")
