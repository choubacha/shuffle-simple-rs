# frozen_string_literal: true

deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
shuffled = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]

shuffles = 0
start = Time.now

loop do
  shuffled.shuffle!
  shuffles += 1
  if shuffled == deck
    puts "We did it! It only took #{shuffles} shuffles"
    break
  elsif (shuffles % 10_000_000).zero?
    rate = shuffles / (Time.now.to_f - start.to_f)
    puts "#{shuffles} shuffles later... at #{rate} shuffles/sec"
  end
end
