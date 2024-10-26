package io.hanbings.server.starplex.data;

import jakarta.servlet.http.HttpServletRequest;

import java.io.IOException;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.Map;

public record RequestTrace(
        String traceId,
        Map<String, String> headers,
        Map<String, String> params,
        Map<String, String> query,
        String method,
        String path,
        String body,
        boolean bodyParsed
) {
    @SuppressWarnings("all")
    public static RequestTrace parse(String traceId, HttpServletRequest request) {
        Map<String, String> headers = new HashMap<>();
        Enumeration<String> headerNames = request.getHeaderNames();
        while (headerNames.hasMoreElements()) {
            String name = headerNames.nextElement();
            headers.put(name, request.getHeader(name));
        }

        Map<String, String> params = new HashMap<>();
        Enumeration<String> parameterNames = request.getParameterNames();
        while (parameterNames.hasMoreElements()) {
            String name = parameterNames.nextElement();
            params.put(name, request.getParameter(name));
        }

        Map<String, String> query = new HashMap<>();
        Enumeration<String> queryNames = request.getParameterNames();
        while (queryNames.hasMoreElements()) {
            String name = queryNames.nextElement();
            query.put(name, request.getParameter(name));
        }

        try {
            return new RequestTrace(
                    traceId,
                    headers,
                    params,
                    query,
                    request.getMethod(),
                    request.getRequestURI(),
                    request.getReader() != null ? request.getReader().readLine() : null,
                    true
            );
        } catch (IOException e) {
            return new RequestTrace(
                    traceId,
                    headers,
                    params,
                    query,
                    request.getMethod(),
                    request.getRequestURI(),
                    null,
                    false
            );
        }
    }
}