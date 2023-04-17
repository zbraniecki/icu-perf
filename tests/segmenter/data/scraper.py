import wikipedia
wikipedia.set_lang("my") # en, ar, zh, ru, th, my
wiki = wikipedia.page("ChatGPT", auto_suggest=False)
text = wiki.content
print(text)
