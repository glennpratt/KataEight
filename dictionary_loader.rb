class DictionaryLoader

  def self.load_file(file_name, mode = :plain)
    words = []
    case mode
    when :plain
      IO.foreach(file_name) {|x| words << x }
    end
    words
  end

end