tags:: tips, azure, speech-to-text, voice-input, productivity

## CLI使用语音输入

[CLI语音输入脚本](../py-playground/speech-to-text-asure/README.md)

其中包括三个python脚本例子，
- 最长三十秒的语音输入的例子;
- 第二是持续输入的例子，没有时间限制。
- 还有一个是文字转语音的例子。

使用例子中的脚步前,
需要先开通azure云的语音服务(speech-service)
然后安装

```
pip install azure-cognitiveservices-speech
```

将上面的脚本`speech-to-text.py`复制到`$PATH` 路径下并可执行权限: `chmod +x speech-to-text.py`
执行即可接受语音并将转换后的文字输出到stdout.


## vim中使用语音输入

然后在`.vimrc`文件里面增加以下key mapping, 使得`alt-d` 可以开启语音输入。

```
inoremap <M-d> <C-r>=system('speech-to-text.py')<CR>
```

vim中, **insert** 模式下, 按下 `Alt-d`, 开始从 microphone 接受语音, 停止说话后,
返回转换后的文字填充在光标出.

## 参考文档

csdn上一个提供较多例子的教程:
https://blog.csdn.net/as604049322/article/details/121059626


Azure speech to text的online demo:
https://speech.microsoft.com/portal/speechtotexttool

azure 语音服务文档:
https://learn.microsoft.com/zh-cn/azure/ai-services/speech-service/how-to-recognize-speech?pivots=programming-language-python

Azure speech to text的官方文档(get started):
https://docs.azure.cn/zh-cn/ai-services/speech-service/get-started-speech-to-text?tabs=macos%2Cterminal&pivots=programming-language-python

Azure的speech to text的其他更多的例子，git repo:
https://github.com/Azure-Samples/cognitive-services-speech-sdk
https://github.com/Azure-Samples/cognitive-services-speech-sdk/tree/master/samples/python/console
https://github.com/Azure-Samples/cognitive-services-speech-sdk/blob/master/samples/python/console/speech_sample.py
https://github.com/Azure-Samples/cognitive-services-speech-sdk/blob/master/samples/python/console/speech_language_detection_sample.py
https://github.com/Azure-Samples/cognitive-services-speech-sdk/blob/master/samples/python/console/meeting_transcription_sample.py


speech Package
https://learn.microsoft.com/en-us/python/api/azure-cognitiveservices-speech/azure.cognitiveservices.speech?view=azure-python

AudioConfig.FromStreamInput:
https://learn.microsoft.com/en-us/dotnet/api/microsoft.cognitiveservices.speech.audio.audioconfig.fromstreaminput?view=azure-dotnet

SpeechConfig Class:
https://learn.microsoft.com/en-us/python/api/azure-cognitiveservices-speech/azure.cognitiveservices.speech.speechconfig?view=azure-python


Supported languages:
https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt
https://cloud.google.com/speech-to-text/docs/speech-to-text-supported-languages
