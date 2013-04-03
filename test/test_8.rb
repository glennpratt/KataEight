require 'test/unit'
#require '8.rb'

class KataEightTest < Test::Unit::TestCase
  def setup
    @dictionary = [
      "a",
      "al",
      "bum",
      "album"
    ]
  end

  def test_kata_eight
    require 'pp'
    pp @dictionary
    assert false
  end
end
