# frozen_string_literal: true

def find_parts(matrix, row, col)
  directions = [
    [-1, 0],  # up
    [0, 1],   # right
    [1, 0],   # down
    [0, -1],  # left

  ]

  diagonals = [
    [-1, -1], # upper left diagonal
    [-1, 1],  # upper right diagonal
    [1, -1],  # lower left diagonal
    [1, 1]    # lower right diagonal
  ]

  output_dirs =  directions.map do |x, y|
    r, c = row + x, col + y

    next unless r.between?(0, matrix.size - 1)
    next unless c.between?(0, matrix[0].size - 1)

    next unless /^\d+$/.match?(matrix[r][c])

    match = matrix[r][c]
    left  = []
    right = []

    (c - 1).downto(0) do |i|
      break unless matrix[r][i] =~ /\d/

      left << matrix[r][i]
    end

    (c + 1).upto(matrix[0].size - 1) do |i|
      break unless matrix[r][i] =~ /\d/

      right << matrix[r][i]
    end

    left.reverse.join + match + right.join
  end

  output_diagonals = diagonals.map do |x, y|
    r, c = row + x, col + y

    next unless r.between?(0, matrix.size - 1)
    next unless c.between?(0, matrix[0].size - 1)

    next if matrix[r][c] !~ /^\d+$/
    next if matrix[r][col] =~ /\d/

    match = matrix[r][c]
    left  = []
    right = []

    if [x, y] == [-1, -1] || [x, y] == [1, -1]
      (c - 1).downto(0) do |i|
        break unless matrix[r][i] =~ /\d/

        left << matrix[r][i]
      end
    end

    if [x, y] == [-1, 1] || [x, y] == [1, 1]
      (c + 1).upto(matrix[0].size - 1) do |i|
        break unless matrix[r][i] =~ /\d/

        right << matrix[r][i]
      end
    end

    [left, right].any?(&:any?) ? left.reverse.join + match + right.join : match
  end

  (output_diagonals + output_dirs).compact
end

def find_gears(matrix, row, col)
  gears = find_parts(matrix, row, col)

  gears.map(&:to_i).reduce(:*) if gears.length > 1
end

def part1(matrix)
  parts = []

  matrix.map.with_index do |row, row_idx|
    next if row.join.scan(/[^.\d]/).empty?

    row.map.with_index do |char, column_idx|
      next unless /[^.\d]/.match?(char)

      parts << find_parts(matrix, row_idx, column_idx)
    end
  end

  parts.flatten.map(&:to_i).sum
end

def part2(matrix)
  gears = []

  matrix.map.with_index do |row, row_idx|
    next if row.join.scan("*").empty?

    row.map.with_index do |char, column_idx|
      next unless /[*]/.match?(char)

      gears << find_gears(matrix, row_idx, column_idx)
    end
  end

  gears.flatten.compact.sum
end

matrix = File.read("input_3.txt").split("\n").map(&:chars)
p part1(matrix) #=> 540212
p part2(matrix) #=> 87605697
