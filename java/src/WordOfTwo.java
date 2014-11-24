import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.Collection;
import java.util.HashSet;

public class WordOfTwo {

    private Collection<String> dict = new HashSet<String>();

    public Collection<String> find() {
        Collection<String> ret = new HashSet<String>();
        for (String word : dict) {
            if (word.length() != 6)
                continue;
            for (int i = 2; i <= 4; i++) {
                if (dict.contains(word.substring(0, i)) && dict.contains(word.substring(i))) {
                    ret.add(word);
                    break;
                }
            }
        }
        return ret;
    }

    public void loadFile(String path) throws IOException {
        BufferedReader reader = new BufferedReader(new FileReader(path));
        String aux;
        while ((aux = reader.readLine()) != null) {
            dict.add(aux);
        }
    }

    public static void main(String[] args) {
        // for (int i = 0; i < 2; i++) {
        test();
      // }
    }

    public static void test() {
        WordOfTwo dictionary = new WordOfTwo();

        long startTime = System.nanoTime();
        try {
            dictionary.loadFile("../assets/wordsEn.txt");
        } catch (IOException e) {
          throw new RuntimeException("Failed to open dictionary.", e);
        }

        long searchTime = System.nanoTime();
        Collection<String> match = dictionary.find();
        long endTime = System.nanoTime();
        
        //System.out.println(match);

        System.out.println("Load Time: " + (searchTime - startTime) + " ns");
        System.out.println("Search Time: " + (endTime - searchTime) + " ns");
        System.out.println("Total Time: " + (endTime - startTime) + " ns");
        
        int expected = 3715;
        int count = match.size();
        System.out.println("Found: " + count + " words");
        
        if (count != expected)
          throw new RuntimeException("Expected " + expected + " matches, got " + count + " matches.");
    }
}
