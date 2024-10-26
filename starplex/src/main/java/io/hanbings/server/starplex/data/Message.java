package io.hanbings.server.starplex.data;

public record Message() {
    public static class Messages {
        public static final String SUCCESS = "好欸！ ╭(○｀∀´○)╯";
        public static final String BAD_REQUEST = "不要乱来啦！ ⁽⁽(੭ꐦ •̀Д•́ )੭*⁾⁾";
        public static final String UNAUTHORIZED = "你还没有登录！ Σ(•̀ω•́ﾉ)ﾉ";
        public static final String NOT_FOUND = "找不到这个东西！ ( ´◔ ‸◔`)";
        public static final String SERVER_ERROR = "服务器出错了！ (◍ ´꒳` ◍)";
        public static final String EMAIL_EXIST = "邮箱已经存在！ (╯°Д°)╯︵ ┻━┻";
        public static final String EMAIL_VERIFY_INVALID = "邮箱验证失败！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_CLIENT_REDIRECT_URI_NOT_MATCH = "回调地址不匹配！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_CLIENT_REDIRECT_URI_INVALID = "回调地址无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_CLIENT_ID_INVALID = "客户端 ID 无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_CLIENT_SCOPE_INVALID = "客户端权限范围无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_AUTHORIZE_CODE_INVALID = "授权码无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_STATE_INVALID = "状态码无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_CLIENT_SECRET_INVALID = "客户端密钥无效！ (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_PROVIDER_INVALID = "第三方登录出现问题 (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_PROVIDER_REQUEST_NETWORK_EXCEPTION = "请求第三方登录出现网络问题 (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_NOT_FOUND = "没有找到对应的 OAuth 应用 (╯°Д°)╯︵ ┻━┻";
        public static final String OAUTH_AUTHORIZE_INVALID = "授权失败 (╯°Д°)╯︵ ┻━┻";
        public static final String EMAIL_FORMAT_INVALID = "邮箱格式不正确 (╯°Д°)╯︵ ┻━┻";
    }

    public static class ReturnCode {
        public static final int SUCCESS = 200;
        public static final int BAD_REQUEST = 400;
        public static final int UNAUTHORIZED = 401;
        public static final int NOT_FOUND = 404;
        public static final int SERVER_ERROR = 503;
        public static final int MAIL_EXIST = 20200;
        public static final int MAIL_VERIFY_INVALID = 20201;
        public static final int OAUTH_CLIENT_REDIRECT_URI_NOT_MATCH = 20202;
        public static final int OAUTH_CLIENT_REDIRECT_URI_INVALID = 20203;
        public static final int OAUTH_CLIENT_ID_INVALID = 20204;
        public static final int OAUTH_CLIENT_SCOPE_INVALID = 20205;
        public static final int OAUTH_AUTHORIZE_CODE_INVALID = 20206;
        public static final int OAUTH_CLIENT_SECRET_INVALID = 20207;
        public static final int OAUTH_STATE_INVALID = 20208;
        public static final int OAUTH_PROVIDER_INVALID = 20209;
        public static final int OAUTH_PROVIDER_REQUEST_NETWORK_EXCEPTION = 20210;
        public static final int OAUTH_PROVIDER_REQUEST_EXCEPTION = 20211;
        public static final int OAUTH_NOT_FOUND = 20212;
        public static final int OAUTH_AUTHORIZE_INVALID = 20213;
        public static final int EMAIL_FORMAT_INVALID = 20214;
    }
}
