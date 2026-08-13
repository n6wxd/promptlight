I'm interested in building an application that works on the desktop similar to something like Spotlight. I will want this for Mac. I want it to be able to deploy to other systems as well. I do have a technology that I'm interested in creating. What I'm looking to work with you to create is a PRD.

Now, again, keep it straight: the PRD is about the *what*, not the *how*. Only in unique scenarios do I want to talk about any version of technology. I don’t really want to get deep into any technology or spec-level specificity of the application tech architecture. The exception is that I want to use something called Tauri, which I’ve used in the past. It’s a desktop enabler for web applications.

I think there is a default component we can use that helps replicate the always-on-top, non-disruptive Spotlight pattern.

One of the things I’m definitely interested in is this Spotlight pattern. It looks like a native Windows system. There’s a custom React component. I’ll let you look through it — I think you can probably find it better than I can.

What I’m looking to build is this: I want an application that I can pop forward — basically a dictionary, a spell dictionary, kind of a spell-not-quite-checker. Oftentimes I can’t spell a word, and I want to very quickly, with a hotkey, pull open a Spotlight-like panel that can help me spell a word.

I want to be able to type a word in and have it very quickly start helping me. I’m sure there are third-party libraries for dictionaries or similar data structures that can look up proper spellings. I’m interested in using third-party solutions for this because I’ll need a solid English dictionary of some kind.

It may be that we use an online service. Some API that can do this. That might not be as quick, but it’s okay if that’s what we need. If we do need an online service, I’d likely use a mini or micro model from OpenAI. That might be the fastest route if we’re not pulling a dictionary locally.

One of the things the app does is allow me to type a word, and if that word is not spelled properly, it starts showing words that I can select from. If I select one of those words, it puts it on my clipboard. Bob’s your uncle. Really, it’s mostly there so I can quickly spell words.

But there’s another concept here too. I’d like to know the meaning of the word. While I’m typing through a word — let’s say it’s a seven-letter word and I’m four letters in — when I click that fifth letter, it has to look up a word and present it as the potential word.

I’m thinking along the lines of a left and right panel in the results window. On the left side, I see words showing up. If I mouse down to select one of them — or the first one is always auto-selected — the right side shows a nice, solid, clean definition.

This gets into where definitions come from. That’s an interesting technical challenge. If you know of clean solutions that are free and fast, that feels like a *what*, not a *how*. If we don’t, AI is always available. I want to clearly offer the opportunity for AI to do this work. AI can do almost all of this with just an API key and a micro or mini model, giving fast solutions for most cases.

To me, that sounds smart — though I’d still prefer things locally if possible. Debouncing and similar considerations are technical and belong in implementation, not the PRD.

So now we have a quick prompt mechanism: hotkey to open, escape to dismiss, enter to select, arrow keys to move through options. I’d like the definition — especially if using AI — to be clean and authoritative. My personal preference is something close to the OED style: solid definition, pronunciation, derivation, origin, dates.

I’m very interested in etymology. Having that information might create fascination. I might want to favorite words. Maybe the system also shows a word of the day, or multiple words of the day, especially when first opened.

This is becoming a word-friendly Spotlight whose focus is helping me learn vocabulary. Some results might not just be dictionary entries but also thesaurus-like information.

Here it gets a bit weird. I may also want Wikipedia-style information. It doesn’t have to be Wikipedia specifically, but an API to Wikipedia could work. AI can also serve as one of the information engines, or even the primary one initially.

I might type in something like IBM. IBM isn’t really a dictionary word, so the system shouldn’t force dictionary mode. Instead, it swaps to information mode. On the left side, instead of spelling suggestions, I might see multiple interpretations — different meanings of IBM, for example.

So we’re moving from strict dictionary lookup to contextual exploration. This suggests that maybe we don’t want to rely only on a dictionary as soon as context enters the picture.

I want to be able to look something up, define something, explore something, learn about something. That leads naturally to a two-panel layout: quick selections on the left via arrow keys, and one-to-many display panels on the right for different kinds of information.

Sometimes the thing I’m looking up is a person. In that case, maybe one panel is about the person. If they’re also a business leader, maybe another panel is about their business work. That implies potentially multiple calls into AI.

The first goal would be to identify all the potential answers, interpretations, or contexts around a term. For example, if I type Reed Hoffman, I might get:

Information about him as a person.

Information about LinkedIn and its creation.

Information about philanthropic work.

Links to books, websites, or external references.

If we’re getting information back from AI, I’d want a strong prompt that asks for structured responses. Not necessarily JSON, but at least clearly structured sections so we know how many pieces of information exist, what kinds they are, and how many panels we might need to display.

That’s one of the core concepts.

So what I’m looking for is a strong *what*-focused PRD around this. It doesn’t need to be massive or endlessly complex. I want a clean, considered, well-thought-through interaction pattern between these pieces.
