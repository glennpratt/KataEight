require File.expand_path(File.dirname(__FILE__)) + '/dictionary_loader.rb'
require File.expand_path(File.dirname(__FILE__)) + '/words.rb'

dictionary = DictionaryLoader.load_file("wordsEn.txt")
words = Words.new(dictionary)
words.beginning_with_two_words
