import azure.cognitiveservices.speech as speechsdk
from azure.cognitiveservices.speech import AudioDataStream, SpeechConfig, SpeechSynthesizer, SpeechSynthesisOutputFormat
from azure.cognitiveservices.speech.audio import AudioOutputConfig

speech_key, service_region = "ad9b32fe8c684dfc913672ce6bae4a3b", "eastasia"

speech_config = speechsdk.SpeechConfig(
    subscription=speech_key,
    region=service_region,
    speech_recognition_language="zh-cn")

speech_config.speech_synthesis_language = "zh-cn"

audio_config = AudioOutputConfig(use_default_speaker=True)
speech_synthesizer = SpeechSynthesizer(
    speech_config=speech_config, audio_config=audio_config)

text_words = "微软人工智能服务非常好用。"
result = speech_synthesizer.speak_text_async(text_words).get()
if result.reason != speechsdk.ResultReason.SynthesizingAudioCompleted:
    print(result.reason)
