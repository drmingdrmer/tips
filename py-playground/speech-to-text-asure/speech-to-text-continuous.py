#!/usr/bin/env python
# coding: utf-8

import os
import time

import azure.cognitiveservices.speech as speechsdk

speech_key = os.getenv('XP_SEC_AZURE_SPEECH_KEY')
region     = os.getenv('XP_SEC_AZURE_SERVICE_REGION')

assert speech_key is not None
assert region is not None


my_phrases = [
        'Raft',
        'Paxos',
]


speech_config = speechsdk.SpeechConfig(
        subscription=speech_key,
        region=region,
        speech_recognition_language="zh-cn")

#  audio_config = speechsdk.audio.AudioConfig(filename=weatherfilename)
#  reco = speechsdk.SpeechRecognizer(speech_config=speech_config, audio_config=audio_config)

reco = speechsdk.SpeechRecognizer(speech_config=speech_config)


phrases = speechsdk.PhraseListGrammar.from_recognizer(reco)
for p in my_phrases:
    phrases.addPhrase(p)


done = False

def stop_cb(evt):
    print('CLOSING on {}'.format(evt))
    reco.stop_continuous_recognition()
    #  nonlocal done
    #  done = True

#  reco.recognizing.connect(lambda evt: print('RECOGNIZING: {}'.format(evt)))
reco.recognized.connect(lambda evt: print('RECOGNIZED: {}'.format(evt)))
#  reco.session_started.connect(lambda evt: print('SESSION STARTED: {}'.format(evt)))
#  reco.session_stopped.connect(lambda evt: print('SESSION STOPPED {}'.format(evt)))
#  reco.canceled.connect(lambda evt: print('CANCELED {}'.format(evt)))

reco.session_stopped.connect(stop_cb)
reco.canceled.connect(stop_cb)

reco.start_continuous_recognition()
while not done:
    time.sleep(.5)


result = reco.recognize_once_async().get()
print(result.text)

#  result = reco.recognize_once()
#  print(result.text, end="")
