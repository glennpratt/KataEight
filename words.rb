class Words

  def initialize(dictionary)
    @words = Hash[*dictionary]
  end

  def beginning_with_two_words
    @words.each do |word|
      if is_word?(word)
        leads = find_words_that_begin_with(word)
        leads.each do |lead|
          r = remainder(word, lead)
          if is_in_dictionary?(r)
            puts 'Match: ' + lead
          end
        end
      end
    end
  end

  def is_in_dictionary?(word)
    return true if @words[word]
  end

  def remainder(lead, word)
    word[lead.length..-1].to_s
  end

  def find_words_that_begin_with(word)
    @words.select { |w| w.start_with?(word) }
  end

  def is_word?(word)
    word.length >= 2
  end

end
