require IEx

defmodule Day1.Common do
  def read_input(file_path) do
    case File.read(file_path) do
      {:ok, data} ->
        String.split(data, "\n", trim: true)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end

  def combine(nil, nil), do: nil

  def combine(int1, int2) do
    int1 = if int1, do: int1
    int2 = if int2, do: int2, else: int1

    str = Integer.to_string(int1) <> Integer.to_string(int2)
    String.to_integer(str)
  end
end

defmodule Day1.Part1 do
  def solve(data) do
    result = Enum.map(data, fn x -> find_integers(x) end)
    Enum.sum(result)
  end

  def find_integers(string) do
    ints = Regex.scan(~r/\d/, string) |> Enum.map(fn [num] -> String.to_integer(num) end)

    int1 = List.first(ints)
    int2 = List.last(ints)

    Day1.Common.combine(int1, int2)
  end
end

defmodule Day1.Part2 do
  def solve(data) do
    result = Enum.map(data, &find_integers/1)
    Enum.sum(result)
  end

  def find_integers(string) do
    fixed_string = replace_strings(string)

    ints =
      Regex.scan(~r/\d/, fixed_string)
      |> Enum.map(fn [num] -> String.to_integer(num) end)

    int1 = List.first(ints)
    int2 = List.last(ints)

    Day1.Common.combine(int1, int2)
  end

  def replace_strings(str) do
    str
    |> String.replace("one", "one1one")
    |> String.replace("two", "two2two")
    |> String.replace("three", "three3three")
    |> String.replace("four", "four4four")
    |> String.replace("five", "five5five")
    |> String.replace("six", "six6six")
    |> String.replace("seven", "seven7seven")
    |> String.replace("eight", "eight8eight")
    |> String.replace("nine", "nine9nine")
  end
end

data = Day1.Common.read_input("input_1.txt")
# => 54644
IO.puts("Part 1 Points: #{Day1.Part1.solve(data)}")
# => 53348
IO.puts("Part 2 Points: #{Day1.Part2.solve(data)}")
