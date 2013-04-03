class Words

  def initialize(dictionary)
    @words = dictionary
  end

  def beginning_with_two_words(words)
    words.each do |word|
      if is_word?(word)
        leads = find_words_this_begins_with(word, words)
        puts 'the leads for ' + word + ': ' + leads.to_s

      end
    end
  end

  def self.find_words_this_begins_with(word, words)
    words.select { |w| w.start_with?(word) }
  end

  def self.is_word?(word)
    word.length >= 2
  end

end

dictionary = DictionaryLoader.load_file("wordsEn.txt")
Words.beginning_with_two_words(dictionary)