# frozen_string_literal: true

require "pry"

def part1(data)
  time, distance = data
  h = time.zip(distance).to_h

  h.map { |t, d| t.downto(0).count { |button_t| button_t * (t - button_t) > d } }.reduce(:*)
end

def part2(data)
  time, distance = data.map{ |i| i.join.to_i }
  acc = 0
  time.downto(0) { |button_t| button_t * (time - button_t) > distance ? acc += 1 : next }
  acc
end

data = File.read("input_6.txt").split("\n").map { |l| l.split.grep(/\A\d+\z/).map(&:to_i) }
p part1(data) #=> 512295
p part2(data) #=> 36530883
