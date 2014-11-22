require 'test/unit'
require File.expand_path(File.dirname(__FILE__)) + '/../dictionary_loader.rb'
require File.expand_path(File.dirname(__FILE__)) + '/../words.rb'
require File.expand_path(File.dirname(__FILE__)) + '/../8.rb'


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
    finder = ConcatWordFinder.new()
    require 'pp'
    pp finder.find
    assert false
  end
end
