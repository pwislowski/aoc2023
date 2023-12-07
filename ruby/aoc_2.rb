# frozen_string_literal: true

require "pry"

def part1(data)
  data.map do |l|
    if [l.gsub(/Game \d+: /, "")].grep(/.*/) do |g|
      (g.scan(/(\d+)\sred/).flatten.map(&:to_i).max&.<= 12 || true) &&
      (g.scan(/(\d+)\sgreen/).flatten.map(&:to_i).max&.<= 13 || true) &&
      (g.scan(/(\d+)\sblue/).flatten.map(&:to_i).max&.<= 14 || true)
    end.first
      l.scan(/(?<=Game )\d+/).map(&:to_i).first
    else
      0
    end
  end.sum
end

def part2(data)
  data.map do |l|
    [l.gsub(/Game \d+: /, "")].grep(/.*/) do |g|
      [
        g.scan(/(\d+)\sred/).flatten.map(&:to_i).max,
        g.scan(/(\d+)\sgreen/).flatten.map(&:to_i).max,
        g.scan(/(\d+)\sblue/).flatten.map(&:to_i).max
      ].compact.reduce(:*)
    end
  end.flatten.sum
end

data = File.read("input_2.txt", trim: true).split("\n")

p part1(data)
p part2(data)
