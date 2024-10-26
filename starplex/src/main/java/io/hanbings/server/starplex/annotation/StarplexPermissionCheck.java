package io.hanbings.server.starplex.annotation;

import com.fasterxml.jackson.databind.JavaType;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.hanbings.server.starplex.data.Token;
import io.hanbings.server.starplex.exception.UnauthorizationException;
import io.hanbings.server.starplex.security.Header;
import io.hanbings.server.starplex.security.Permission;
import io.hanbings.server.starplex.service.TokenService;
import io.hanbings.server.starplex.utils.RandomUtils;
import jakarta.servlet.http.HttpServletRequest;
import lombok.RequiredArgsConstructor;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.reflect.MethodSignature;
import org.springframework.stereotype.Component;
import org.springframework.web.context.request.RequestContextHolder;
import org.springframework.web.context.request.ServletRequestAttributes;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Set;

@Target({ElementType.METHOD})
@Retention(RetentionPolicy.RUNTIME)
@SuppressWarnings("SpellCheckingInspection")
public @interface StarplexPermissionCheck {
    String[] access();

    boolean requiredLogin() default true;

    boolean requiredAllAccess() default true;

    @Slf4j
    @Aspect
    @Component
    @RequiredArgsConstructor
    @SuppressWarnings("all")
    public class AccessChecker {

        static ObjectMapper mapper = new ObjectMapper();
        static JavaType listType = mapper.getTypeFactory().constructParametricType(ArrayList.class, Permission.class);
        final TokenService tokenService;

        @SneakyThrows
        @Around(value = "@annotation(io.hanbings.server.starplex.annotation.StarplexPermissionCheck)")
        public Object check(ProceedingJoinPoint point) {
            // get the permissions that are specified by the annotation
            MethodSignature signature = (MethodSignature) point.getSignature();
            Object target = point.getTarget();
            Method method = target.getClass().getMethod(signature.getName(), signature.getParameterTypes());
            HttpServletRequest request = ((ServletRequestAttributes) RequestContextHolder.getRequestAttributes()).getRequest();
            StarplexPermissionCheck annotation = method.getAnnotation(StarplexPermissionCheck.class);

            // get Authorization header
            String authorization = request.getHeader("Authorization");

            // check if login is required
            if (annotation.requiredLogin() && authorization == null)
                throw new UnauthorizationException(RandomUtils.uuid(), request.getRequestURI());


            Token token = tokenService.parse(authorization);

            // check if login is expired
            if (annotation.requiredLogin() && token == null)
                throw new UnauthorizationException(RandomUtils.uuid(), request.getRequestURI());

            // check if all permissions are required
            Set<String> willBeCheck = Set.of(annotation.access());

            if (!annotation.requiredAllAccess()) {
                if (!tokenService.checkAccess(token, willBeCheck)) {
                    throw new UnauthorizationException(RandomUtils.uuid(), request.getRequestURI());
                }
            } else {
                for (String permission : willBeCheck) {
                    if (!tokenService.checkAccess(token, permission)) {
                        throw new UnauthorizationException(RandomUtils.uuid(), request.getRequestURI());
                    }
                }
            }

            // set id to attribute
            request.setAttribute(Header.ACCOUNT, token.belong());
            if (token != null) {
                request.setAttribute(Header.CUSTOM_HEADER, token);
            }

            return point.proceed();
        }

        private Class<?>[] getParameterTypes(ProceedingJoinPoint point) {
            Object[] args = point.getArgs();
            Class<?>[] parameterTypes = new Class<?>[args.length];
            for (int i = 0; i < args.length; i++) {
                parameterTypes[i] = args[i].getClass();
            }
            return parameterTypes;
        }
    }
}