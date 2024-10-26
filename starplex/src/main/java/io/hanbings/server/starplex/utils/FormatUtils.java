package io.hanbings.server.starplex.utils;

public class FormatUtils {
    final static String regex = "\\b[\\w.%-]+@[\\w.-]+\\.[A-Za-z]{2,}\\b";

    public static boolean checkEmail(String email) {
        return email.matches(regex);
    }
}
