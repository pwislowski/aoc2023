require IEx

defmodule Day6.Common do
  def read_input(file_path) do
    case File.read(file_path) do
      {:ok, data} ->
        String.split(data, "\n", trim: true)
        |> Enum.map(fn line ->
          line
          |> String.split()
          |> Enum.drop(1)
          |> Enum.map(&String.to_integer/1)
        end)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end
end

defmodule Day6.Part1 do
  def solve(data) do
    [time, distance] = data

    time
    |> Enum.zip(distance)
    |> Enum.map(fn {t, d} ->
      0..t
      |> Enum.filter(fn button_t -> button_t * (t - button_t) > d end)
      |> Enum.count()
    end)
    |> Enum.reduce(&*/2)
  end
end

defmodule Day6.Part2 do
  def solve(data) do
    [time, distance] =
      data
      |> Enum.map(fn x ->
        x
        |> Enum.map(&Integer.to_string/1)
        |> Enum.join()
        |> String.to_integer()
      end)

    Enum.reduce(0..time, 0, fn bt, acc ->
      if bt * (time - bt) > distance do
        acc + 1
      else
        acc
      end
    end)
  end
end

data = Day6.Common.read_input("input_6.txt")
# => 512295
IO.puts("Part 1: #{Day6.Part1.solve(data)}")
# # => 36530883
IO.puts("Part 2 : #{Day6.Part2.solve(data)}")
