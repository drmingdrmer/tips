#!/usr/bin/env python
# coding: utf-8

import os

import azure.cognitiveservices.speech as speechsdk

speech_key = os.getenv('XP_SEC_AZURE_SPEECH_KEY')
region     = os.getenv('XP_SEC_AZURE_SERVICE_REGION')

assert speech_key is not None
assert region is not None

speech_config = speechsdk.SpeechConfig(
        subscription=speech_key,
        region=region,
        speech_recognition_language="zh-cn")

reco = speechsdk.SpeechRecognizer(speech_config=speech_config)

my_phrases = [
        'Raft',
        'Paxos',
]

phrases = speechsdk.PhraseListGrammar.from_recognizer(reco)
for p in my_phrases:
        phrases.addPhrase(p)

result = reco.recognize_once_async().get()
print(result.text)

#  result = reco.recognize_once()
#  print(result.text, end="")
