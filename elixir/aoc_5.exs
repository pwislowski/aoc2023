require IEx

defmodule Day5.Common do
  def read_input(file_path) do
    case File.read(file_path) do
      {:ok, data} ->
        String.split(data, "\n\n", trim: true)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end

  def parse_almanac(data) do
    data
    |> Enum.map(fn x ->
      x
      |> String.split("\n")
      |> Enum.drop(1)
      |> Enum.reject(&(&1 == ""))
      |> Enum.map(fn set ->
        set
        |> String.split(" ")
        |> Enum.map(&String.to_integer/1)
      end)
    end)
  end
end

defmodule Day5.Part1 do
  def solve(data) do
    init_seeds = parse_init_seeds(data)
    data = Enum.drop(data, 1)

    almanac = Day5.Common.parse_almanac(data)

    Enum.map(init_seeds, fn seed ->
      find_localization(seed, almanac)
    end)
    |> Enum.min()
  end

  def find_localization(seed, almanac) do
    Enum.reduce(almanac, seed, fn maps, value ->
      find_value(value, maps)
    end)
  end

  def find_value(value, maps) do
    Enum.reduce_while(maps, value, fn [dest, source, range], acc ->
      if acc in source..(source + range - 1) do
        {:halt, dest + (acc - source)}
      else
        {:cont, acc}
      end
    end)
  end

  def parse_init_seeds(data) do
    data
    |> List.first()
    |> String.split(": ")
    |> Enum.drop(1)
    |> hd()
    |> String.split(" ")
    |> Enum.map(&String.to_integer/1)
  end
end

defmodule Day5.Part2 do
  def solve(data) do
    init_seeds = parse_init_seeds(data)

    almanac = Day5.Common.parse_almanac(data)

    val = Enum.map(init_seeds, fn x ->
      nums = Enum.to_list(x)

      Enum.map(nums, fn seed ->
        IO.puts(seed)
        find_localization(seed, almanac)
      end)
      |> Enum.min()
    end)
    # brute force ftw
    Enum.min(val)
  end

  def find_localization(seed, almanac) do
    Enum.reduce(almanac, seed, fn maps, value ->
      find_value(value, maps)
    end)
  end

  def find_value(value, maps) do
    Enum.reduce_while(maps, value, fn [dest, source, range], acc ->
      if acc in source..(source + range - 1) do
        {:halt, dest + (acc - source)}
      else
        {:cont, acc}
      end
    end)
  end

  def parse_init_seeds(data) do
    data
    |> List.first()
    |> String.split(": ")
    |> Enum.drop(1)
    |> hd()
    |> String.split(" ")
    |> Enum.map(&String.to_integer/1)
    |> Enum.chunk_every(2)
    |> Enum.map(fn [start, length] -> start..(start + length - 1) end)
  end
end

data = Day5.Common.read_input("input_5.txt")
# => 88151870
# IO.puts("Part 1: #{Day5.Part1.solve(data)}")
# =>
IO.puts("Part 2 : #{Day5.Part2.solve(data)}")
