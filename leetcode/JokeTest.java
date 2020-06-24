

public class JokeTest {
    
}

class JokesAPI {
    static final String BASE_URL = "https://official-joke-api.appspot.com/jokes/programming";
    static final String RANDOM_JOKES_URL = BASE_URL + "/random";
    static final String TEN_JOKES_URL = BASE_URL + "/ten";

    public static Joke getRandomJoke() {

    }

    public static List<Joke> getTenJokes() {

    }
}

class Joke {
    int id;
    String type;
    String setup;
    String punchline;
    
    public Joke(int id, String type, String setup, String punchline) {
        this.id = id;
        this.type = type;
        this.setup = setup;
        this.punchline = punchline;
    }

    public String getType() {
        return this.type;
    }

    public int getId() {
        return this.id;
    }
}