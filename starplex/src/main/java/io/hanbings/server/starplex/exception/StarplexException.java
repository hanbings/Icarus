package io.hanbings.server.starplex.exception;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.hanbings.server.starplex.data.Message;
import io.hanbings.server.starplex.data.RequestTrace;
import io.hanbings.server.starplex.utils.RandomUtils;
import io.hanbings.server.starplex.utils.TimeUtils;
import jakarta.servlet.http.HttpServletRequest;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;

import java.util.Arrays;
import java.util.Map;

@Data
@Slf4j
@EqualsAndHashCode(callSuper = true)
@SuppressWarnings("SpellCheckingInspection")
public class StarplexException extends RuntimeException {
    String traceId;
    int code;
    String message;
    long timestamp;

    public StarplexException(String traceId, int code, String message) {
        this.traceId = traceId;
        this.code = code;
        this.message = message;
        this.timestamp = TimeUtils.getMilliUnixTime();
    }

    @ControllerAdvice
    public static class StarplexExceptionHandler {
        static ObjectMapper mapper = new ObjectMapper();

        @SuppressWarnings("all")
        @ExceptionHandler(Exception.class)
        public ResponseEntity<Map> handleMyException(HttpServletRequest request, Exception e) throws JsonProcessingException {
            HttpStatus status = HttpStatus.valueOf(Message.ReturnCode.SERVER_ERROR);
            String traceId = RandomUtils.uuid();
            String message = Message.Messages.SERVER_ERROR;
            long time = TimeUtils.getMilliUnixTime();

            if (e instanceof StarplexException ne) {
                status = HttpStatus.valueOf(ne.code);
                traceId = ne.traceId;
                message = ne.message;
            }

            RequestTrace requestTrace = RequestTrace.parse(traceId, request);
            log.warn(
                    String.format(
                            "Request URL: %s\nMessage: %s\nTime: %s\nRequest Data: %s\nStack Trace: %s",
                            request.getRequestURL(),
                            e.getMessage(),
                            time,
                            mapper.writeValueAsString(requestTrace),
                            Arrays.toString(e.getStackTrace())
                    )
            );

            return new ResponseEntity<>(
                    Map.of(
                            "traceId", traceId,
                            "code", status.value(),
                            "message", message,
                            "timestamp", time
                    ),
                    status
            );
        }
    }
}