# rubocop: disable all

require "pry"

DECK = {
  "A" => 114,
  "K" => 113,
  "Q" => 112,
  "J" => 111,
  "T" => 110,
  "9" => 109,
  "8" => 108,
  "7" => 107,
  "6" => 106,
  "5" => 105,
  "4" => 104,
  "3" => 103,
  "2" => 102
}

HANDS = {
  7 => [], # "5OAK"
  6 => [], # "4OAK"
  5 => [], # "FULL"
  4 => [], # "3OAK"
  3 => [], # "2P"
  2 => [], # "1P"
  1 => [], # "HC"
}

def sort(with_jokes, without_jokers, bid)
  case with_jokes.tally.values.sort.reverse
  in [5, *]
    HANDS[7] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # 5oak
  in [4, *]
    HANDS[6] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # 4oak
  in [3, 2, *]
    HANDS[5] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # full
  in [3, *]
    HANDS[4] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # 3oak
  in [2, 2, *]
    HANDS[3] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # 2p
  in [2, *]
    HANDS[2] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # 1p
  else
    HANDS[1] << [without_jokers.map { |c| DECK.dig(c) }.join.to_i, bid.to_i] # hc
  end
end

def sort_hands(hand)
  cards, bid = hand.split
  splited_cards = cards.chars

  sort(splited_cards, splited_cards, bid)
end

def sort_hands_with_jokers(hand)
  cards, bid      = hand.split
  replaced_jokers = cards.chars
  splited_cards   = cards.chars

  if replaced_jokers.include?("J")
    case replaced_jokers.count("J")
    in 5
      replaced_jokers.map! { |c| c.tr("J", "A") }
    else
      crd = if replaced_jokers.reject { |c| c == "J" }.tally.values.uniq.count == 1
              DECK.detect { |_k, v| v == replaced_jokers.reject { |c| c == "J" }.map { |c| DECK.dig(c) }.max }.first
            else
              replaced_jokers.reject { |c| c == "J" }.tally.max_by { |_k, v| v }[0]
            end

      replaced_jokers.map! { |c| c.tr("J", crd) }
    end
  end

  sort(replaced_jokers, splited_cards, bid)
end

def sort_n_solve
  HANDS.each do |key, value|
    HANDS[key] = value.sort_by { |item| item[0] }.reverse
  end

  acc = 0
  HANDS.values.flatten(1).reverse.each_with_index do |arr, idx|
    acc += (idx + 1) * arr.last
  end
  acc
end

def part1(data)
  data.map do |hand|
    sort_hands(hand)
  end

  sort_n_solve
end

def part2(data)
  HANDS.each_value(&:clear)
  DECK["J"] = 101

  data.map do |hand|
    sort_hands_with_jokers(hand)
  end

  sort_n_solve
end

data = File.readlines("input_7.txt", chomp: true)
p part1(data) #=> 251545216
p part2(data) #=> 250384185
