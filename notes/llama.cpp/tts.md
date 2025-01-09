## Text To Speach (TTS)
In llama.cpp there a new feature has been added that enabled text to speach
and there is an accompanying example named tts.

### tts example
The example in llama.cpp uses a model from [outeai](https://www.outeai.com/)

Download the model that contains the LLM model:
```console
$ pushd models
$ git clone --branch main --single-branch --depth 1 https://huggingface.co/OuteAI/OuteTTS-0.2-500M
$ cd OuteTTS-0.2-500M && git lfs install && git lfs pull
$ popd
```
Convert the model to .gguf format:
```console
(venv) python convert_hf_to_gguf.py models/OuteTTS-0.2-500M/ --outfile models/outetts-0.2-0.5B-f16.gguf --outtype f16
```
The generated model will be `models/outetts-0.2-0.5B-f16.gguf`.

We can optionally quantize this to Q8_0 using the following command:
```console
$ build/bin/llama-quantize models/outetts-0.2-0.5B-f16.gguf models/outetts-0.2-0.5B-q8_0.gguf q8_0
```
The quantized model will be `models/outetts-0.2-0.5B-q8_0.gguf`.

Next we do something simlar to the audio decoder. Recall that this is text to
audio so the generated tokens from the LLM need to be converted into audio.
First download the model for the voice decoder:
```console
$ pushd models
$ git clone --branch main --single-branch --depth 1 https://huggingface.co/novateur/WavTokenizer-large-speech-75token
$ cd WavTokenizer-large-speech-75token && git lfs install && git lfs pull
$ popd
```
This model file is PyTorch checkpoint (.ckpt) and we first need to convert it to
huggingface format:
```console
(venv) python examples/tts/convert_pt_to_hf.py models/WavTokenizer-large-speech-75token/wavtokenizer_large_speech_320_24k.ckpt
...
Model has been successfully converted and saved to models/WavTokenizer-large-speech-75token/model.safetensors
Metadata has been saved to models/WavTokenizer-large-speech-75token/index.json
Config has been saved to models/WavTokenizer-large-speech-75tokenconfig.json
```
And then we can convert the huggingface format to gguf:
```console
(venv) python convert_hf_to_gguf.py models/WavTokenizer-large-speech-75token/ --outfile models/wavtokenizer-large-75-f16.gguf --outtype f16
...
INFO:hf-to-gguf:Model successfully exported to models/wavtokenizer-large-75-f16.gguf
```

With both of the models generated, the LLM model and the voice decoder model,
we can run the example:
```console
$ build/bin/llama-tts -m  ./models/outetts-0.2-0.5B-q8_0.gguf \
    -mv ./models/wavtokenizer-large-75-f16.gguf \
    -p "Hello world"
...
main: audio written to file 'output.wav'
```
The output.wav file will contain the audio of the prompt. This can be heard
by playing the file with a media player. On Linux the following command will
play the audio:
```console
$ aplay output.wav
```

