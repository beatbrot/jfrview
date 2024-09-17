import java.util.Random;
import java.util.stream.IntStream;

public class Heavy {
    public static void main(String[] args) {
        var r = new java.util.Random();
        var iArr = IntStream.generate(r::nextInt)
            .limit(40_000_000)
            .sorted()
            .toArray();
        System.out.println(iArr.length);
    }
}
