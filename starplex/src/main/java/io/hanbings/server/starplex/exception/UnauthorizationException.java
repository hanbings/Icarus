package io.hanbings.server.starplex.exception;

import io.hanbings.server.starplex.data.Message;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@SuppressWarnings("SpellCheckingInspection")
public class UnauthorizationException extends StarplexException {
    @SuppressWarnings("all")
    public UnauthorizationException(String traceId, String path) {
        super(
                traceId,
                Message.ReturnCode.UNAUTHORIZED,
                Message.Messages.UNAUTHORIZED
        );

        log.warn(
                """
                        Trace ID: {traceId}
                        Catch UnauthorizationException: {path}
                        """,
                traceId,
                path
        );
    }
}