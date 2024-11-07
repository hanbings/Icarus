package io.hanbings.server.starplex.service;

import com.google.gson.Gson;
import com.plexpt.chatgpt.ChatGPT;
import com.plexpt.chatgpt.entity.chat.ChatChoice;
import com.plexpt.chatgpt.entity.chat.ChatCompletion;
import com.plexpt.chatgpt.entity.chat.ChatCompletionResponse;
import com.plexpt.chatgpt.entity.chat.Message;
import io.hanbings.server.starplex.config.ChatGPTConfig;
import io.hanbings.server.starplex.data.ChatCPTCountry;
import io.hanbings.server.starplex.data.ChatGPTReadme;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ChatGPTService {
    static ChatGPT chatGPT;
    static Gson gson = new Gson();
    static com.plexpt.chatgpt.entity.chat.Message summarizeReadmePrompt = Message.of("""
            请为我总结一下个人简介，根据文字编码、语言风格、文本内容等多种因素进行总结
            需要注意的内容是：
            1.所在的国家（例如使用中文说明，这种编码就可以认为是中国，但要注意的是使用英文不一定在英国） 
            2.个人博客地址 
            3.对哪一种编程语言的掌握更好 
            4.100 字左右的评价 
            5.给出一个 0 - 100 分的分数（如果我给出的内容为空，则为 0 分）
            
            请以 
            { 
                "description": "评价内容", 
                "country": "所在国家", 
                "blog": "个人博客地址", 
                "language": "掌握的编程语言（如果有多种则以数组形式）", 
                "score": "分数"
            } 
            的格式（纯 json 格式，不要携带 markdown 代码块格式）返回你的回复
            不要回复多余的语句，无法总结出来的部分使用 null 替代
            """
    );

    static com.plexpt.chatgpt.entity.chat.Message searchCountryPrompt = Message.of("""
            请问这个地方位于哪一个国家，可以可能的猜测
            以
            {
                "country": 简体中文全称（如果某个地区属于某个国家，则回复国家全称，比如香港澳门和台湾属于中国）
                "address": 可能的详细地址
                "probability": 0.0 - 1.0 之间的猜测概率
            }
            的格式（纯 json 格式，不要携带 markdown 代码块格式）返回你的回复
            不要回复多余的语句，如果你不知道或是这个地方实际上不存在以未知和 0 概率替代
            """
    );

    public ChatGPTService(ChatGPTConfig chatGPTConfig) {
        chatGPT = ChatGPT.builder()
                .apiKey(chatGPTConfig.apiKey())
                .apiHost(chatGPTConfig.baseAPI())
                .build()
                .init();
    }

    public ChatGPTReadme summarizeReadme(String readme) {
        Message res = getMessage(summarizeReadmePrompt, readme);
        return gson.fromJson(res.getContent(), ChatGPTReadme.class);
    }

    public ChatCPTCountry searchCountry(String address) {
        Message res = getMessage(searchCountryPrompt, address);
        return gson.fromJson(res.getContent(), ChatCPTCountry.class);
    }

    com.plexpt.chatgpt.entity.chat.Message getMessage(
            com.plexpt.chatgpt.entity.chat.Message prompt,
            String message
    ) {
        com.plexpt.chatgpt.entity.chat.Message content = Message.of(message);

        ChatCompletion chatCompletion = ChatCompletion.builder()
                .model(ChatCompletion.Model.GPT_3_5_TURBO)
                .messages(List.of(prompt, content))
                .maxTokens(4096)
                .temperature(0.9)
                .build();

        ChatCompletionResponse response = chatGPT.chatCompletion(chatCompletion);
        ChatChoice choice = response.getChoices().getFirst();

        return choice.getMessage();
    }
}
