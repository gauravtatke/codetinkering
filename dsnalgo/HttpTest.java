import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.net.HttpURLConnection;
import java.net.URL;

public static class HttpTest {
    private static final String USER_AGENT = "Mozilla/5.0";
    private static final String GET_URL = "http://localhost:9090";
    private static final String POST_URL = "http://localhost:9090";

    public static void sendGet() {
        Url obj = new Url(GET_URL);
        HttpURLConnection con = (HttpURLConnection) obj.openConnection();
        con.setRequestMethod("GET");
        con.setRequestProperty("User-Agent", USER_AGENT);
        int responseCode = con.getResponseCode();
        if (responseCode == HttpURLConnection.HTTP_OK) {
            // do something
            BufferedReader br = new BufferedReader(new InputStreamReader(con.getInputStream()));
            String inputLine;
            StringBuffer buffer = new StringBuffer();
            while ((inputLine = br.readLine()) != null) {
                buffer.append(inputLine);
            }
            br.close();
            System.out.println(br);
        } else {
            System.out.println("GET request did not work");
        }
    }

    public static void sendPost() {
        URL obj = new Url(POST_URL);
        HttpURLConnection con = (HttpURLConnection) obj.openConnection();
        con.setRequestMethod("POST");
        con.setRequestProperty("User-Agent", USER_AGENT);
        con.setDoOutput(true); // only needed for post
        OutputStream output = con.getOutputStream();
        output.write("some data".getBytes());
        output.flush();
        output.close();
        int responseCode = con.getResponseCode();
        if (responseCode == HttpURLConnection.HTTP_OK) {
            BufferedReader br = new BufferedReader(new InputStreamReader(con.getInputStream()));
            String inputLine;
            StringBuffer buffer = new StringBuffer();
            while ((inputLine = br.readLine()) != null) {
                buffer.append(inputLine);
            }
            br.close();
            System.out.println(buffer);
        } else {
            System.out.println("Post Request did not work");
        }

    }

    public static void main(String[] args) {
        sendGet();
        sendPost();
    }
}