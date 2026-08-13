# WordLight PRD

## Product Vision

WordLight is a Spotlight-style desktop application for instant word exploration, spelling assistance, and knowledge lookup. It lives in the background and appears instantly via hotkey, helping users quickly spell words, understand definitions, explore etymology, and discover information about people, places, companies, and concepts.

The core promise: **summon knowledge instantly, then disappear**.

---

## User Problems

1. **Spelling uncertainty**: "How do you spell 'accommodate'?" — Users need fast spelling confirmation without opening a browser or app.

2. **Definition curiosity**: "What exactly does 'ephemeral' mean?" — Quick access to clear, authoritative definitions.

3. **Etymology fascination**: "Where does the word 'salary' come from?" — For word lovers who want origin, derivation, and historical context.

4. **Concept lookup**: "Who is Reed Hoffman?" or "What is CRISPR?" — Quick information about people, companies, acronyms, and ideas that aren't dictionary words.

5. **Vocabulary building**: Users who want to expand their vocabulary through daily word discovery and the ability to save interesting words.

---

## Core Experience

### The Spotlight Pattern

WordLight follows the Spotlight interaction model:

- **Summon**: Global hotkey pulls up a centered, floating panel
- **Type**: Begin typing immediately — no clicking required
- **Navigate**: Arrow keys move through results
- **Select**: Enter copies the word to clipboard
- **Dismiss**: Escape closes the panel instantly
- **Invisible return**: Focus returns to wherever the user was working

The entire interaction should take 2-5 seconds for common lookups.

---

## Interface Layout

### Two-Panel Design

```
┌────────────────────────────────────────────────────────────────┐
│  🔍 [apple█]                                                   │
├───────────────────────┬────────────────────────────────────────┤
│                       │                                        │
│  ✓ apple              │  apple                                 │
│    ○ Word             │  /ˈæpəl/                               │
│                       │                                        │
│    apple              │  noun                                  │
│    ○ Company          │  1. the round fruit of a tree of the   │
│                       │     rose family, with red or green     │
│    Apple Inc.         │     skin and crisp flesh               │
│    ○ Company          │                                        │
│                       │  Origin: Old English æppel, of         │
│                       │  Germanic origin                       │
│                       │                                        │
│                       │  ─────────────────────────────         │
│                       │  Synonyms: fruit, pome...              │
│                       │                                        │
├───────────────────────┴────────────────────────────────────────┤
│  Enter copy  |  Tab thesaurus  |  ⌘F favorite  |  Esc dismiss  │
└────────────────────────────────────────────────────────────────┘
```

**Left Panel: Result List**
- Shows matching results as user types
- Each result displays a **type indicator** from the taxonomy (Word, Company, Person, Place, Acronym, etc.)
- Dictionary/Word results appear **first** by default
- Concept results (companies, people, etc.) appear below dictionary results
- User simply arrows down to move past dictionary entries to other contexts
- Highlights currently selected result
- Scrolls with keyboard navigation
- **Enter copies the selected item's term to clipboard** (the word or name itself, not the definition)

**Right Panel: Information Display**
- Shows detailed information for the selected result
- Content format adapts based on result type (definition vs. concept)
- Scrollable for longer content

### Result Type Taxonomy

Each result in the left panel is tagged with a type indicator:

| Type | Examples |
|------|----------|
| **Word** | apple, accommodate, ephemeral |
| **Person** | Reid Hoffman, Marie Curie |
| **Company** | Apple Inc., IBM, OpenAI |
| **Place** | Paris, Mount Everest |
| **Acronym** | NASA, CRISPR, API |
| **Concept** | Blockchain, Existentialism |

**Default ordering**: Dictionary (Word) results first, then other types.

**Future consideration**: Allow users to reorder the taxonomy to prioritize what matters most to them (e.g., a business user might want Company results first).

---

## Feature Set

### 1. Spelling Assistance

**The primary use case.**

As the user types, WordLight shows correctly-spelled words that match the partial input. Even with significant misspellings, the system should surface the intended word.

- Type "accomodate" → shows "accommodate" as top result
- Type "definately" → shows "definitely"
- Type "entrepre" → shows "entrepreneur", "enterprise", "entrepreneurial"

The user can select a word and press Enter to copy the correct spelling to clipboard.

#### Spelling Correction Feedback

When a misspelling is detected, WordLight should **teach** the user what they got wrong — not just show the correct word, but highlight the correction.

**Visual correction indicators:**

```
┌───────────────────────┐
│  ✓ acco[mm]odate      │  ← Highlights the doubled "mm" you missed
│    ○ Word             │
│                       │
│    accommodation      │
│    ○ Word             │
└───────────────────────┘
```

Or with a correction hint:

```
┌───────────────────────┐
│  ✓ accommodate        │
│    ○ Word  · mm not m │  ← Brief note showing the fix
│                       │
│    accommodation      │
│    ○ Word             │
└───────────────────────┘
```

**Examples of correction hints:**
- "accomodate" → "mm not m"
- "definately" → "ite not ate"
- "acommodate" → "cc and mm"
- "seperate" → "a not e"

This transforms a simple spell-check into a **learning moment**. Over time, users internalize the correct spellings rather than just copying them.

### 2. Dictionary Definitions

When a word is selected, the right panel displays an authoritative definition in the style of the Oxford English Dictionary:

- **Pronunciation**: IPA phonetic spelling with audio option
- **Part of speech**: noun, verb, adjective, etc.
- **Definitions**: Numbered, ordered by common usage
- **Example sentences**: Brief usage examples
- **Etymology**: Word origin, language roots, historical evolution
- **First known use**: Date or era when the word entered English

The definition should feel scholarly but accessible — not dumbed down, but not impenetrable.

### 3. Thesaurus

A secondary view (accessible via Tab or a toggle) showing:

- **Synonyms**: Words with similar meaning
- **Antonyms**: Words with opposite meaning
- **Related words**: Conceptually connected terms

Selecting a synonym updates the left panel to show that word's definition.

### 4. Concept Lookup

When the input isn't a dictionary word (or matches a known entity), WordLight switches to concept mode:

**Person lookup**: "Reed Hoffman"
```
┌───────────────────────┬────────────────────────────────────────┐
│  Reed Hoffman         │  Reid Hoffman                          │
│  ● Person             │  Entrepreneur & Investor               │
│                       │                                        │
│  Reid Hoffman         │  Co-founder of LinkedIn. Partner at    │
│  ● LinkedIn           │  Greylock Partners. Author of "The     │
│                       │  Start-up of You" and "Blitzscaling."  │
│  Reid Hoffman         │                                        │
│  ● Investor           │  Born: August 5, 1967                  │
│                       │  Palo Alto, California                 │
│                       │                                        │
└───────────────────────┴────────────────────────────────────────┘
```

**Company/Acronym lookup**: "IBM"
```
┌───────────────────────┬────────────────────────────────────────┐
│  IBM                  │  IBM (International Business Machines) │
│  ● Company            │                                        │
│                       │  American multinational technology     │
│  IBM                  │  company headquartered in Armonk, NY.  │
│  ● Stock              │  Founded 1911. Known for mainframes,   │
│                       │  cloud computing, and AI research.     │
│  IBM                  │                                        │
│  ● History            │  Notable: Watson, Deep Blue, PC        │
│                       │                                        │
└───────────────────────┴────────────────────────────────────────┘
```

The left panel shows different **contexts** or **facets** of the entity. Selecting a facet updates the right panel with relevant information.

### 5. Information Depth

The right panel should provide **mini-Wikipedia depth** — not just a quick paragraph, but genuine exploration material for curious users.

**Structure for all information types:**

1. **TLDR / Overview** (always visible at top)
   - 1-2 sentence quick-glance summary
   - Enough to answer "what is this?" instantly

2. **Full Content** (scrollable below)
   - For **Words**: Full definition, etymology, usage examples, related forms
   - For **People**: Biography, achievements, notable works, dates
   - For **Companies**: History, products, significance, key facts
   - For **Concepts**: Explanation, context, applications, related ideas

3. **Related** (at bottom)
   - Connected concepts, people, or words
   - "See also" style links for deeper exploration

The goal is to satisfy both quick lookups ("how do I spell this?") and genuine curiosity ("tell me more about this"). Users who want depth should find it without leaving the app.

### 6. Favorites

Users can mark words or concepts as favorites:

- Press `Cmd+F` (or similar) to favorite the current selection
- Favorites are accessible from a dedicated view
- Favorites persist across sessions

Use cases:
- Building a personal vocabulary list
- Saving words to learn later
- Collecting interesting etymologies

### 7. Empty State: Discovery Mode

When WordLight is summoned with no query, instead of showing a blank screen, it becomes a **discovery surface** for vocabulary building.

```
┌────────────────────────────────────────────────────────────────┐
│  🔍 [                                                    ]     │
├───────────────────────┬────────────────────────────────────────┤
│                       │                                        │
│  ✦ WORD OF THE DAY    │  serendipity                           │
│                       │  /ˌserənˈdipədē/                       │
│    serendipity        │                                        │
│    ○ Word             │  noun                                  │
│                       │  The occurrence of events by chance    │
│  ───────────────────  │  in a happy or beneficial way.         │
│                       │                                        │
│  ★ FAVORITES          │  Origin: Coined by Horace Walpole in   │
│                       │  1754, from the Persian fairy tale     │
│    ephemeral          │  "The Three Princes of Serendip"       │
│    ○ Word             │                                        │
│                       │                                        │
│    Reid Hoffman       │                                        │
│    ○ Person           │                                        │
│                       │                                        │
├───────────────────────┴────────────────────────────────────────┤
│  ↓ explore  |  ⌘F favorite  |  Esc dismiss                     │
└────────────────────────────────────────────────────────────────┘
```

**Empty state shows:**

1. **Word(s) of the Day** — Interesting, vocabulary-expanding words
   - Refreshes daily
   - Curated for learning value (etymology, uncommon usage, beautiful words)
   - User can set how many words appear (1-3)

2. **Favorites** — User's saved words and concepts
   - Quick access to vocabulary they're building
   - Can browse and revisit saved items
   - Ordered by most recently favorited or alphabetically (user preference)

The empty state reinforces WordLight's purpose as a **vocabulary extender** — not just a spell-checker, but a tool for word lovers who want to grow their language.

### 8. History

Recent lookups are remembered:

- Quick access to previously searched words
- Accessible via a subtle history indicator or keyboard shortcut
- Cleared on demand for privacy
- Does not appear in empty state by default (focus is on discovery, not repetition)

---

## Keyboard Interaction

| Key | Action |
|-----|--------|
| `Cmd+Shift+Space` | Summon WordLight (configurable) |
| `Escape` | Dismiss |
| `↑` / `↓` | Navigate word list |
| `Enter` | Copy selected word to clipboard |
| `Tab` | Toggle between Definition and Thesaurus views |
| `Cmd+F` | Add to favorites |
| `Cmd+C` | Copy definition text |
| `←` / `→` | Navigate information facets (for concept lookups) |

The interface should be fully usable without a mouse.

---

## Information Sources

WordLight requires authoritative, fast information. The system should be able to:

1. **Recognize misspellings** and suggest correct spellings
2. **Provide dictionary definitions** with etymology and pronunciation
3. **Offer thesaurus data** (synonyms, antonyms)
4. **Answer concept queries** about people, places, companies, acronyms

**Considerations:**

- **Speed**: Lookups should feel instant (< 200ms perceived)
- **Offline capability**: Basic spelling/definitions should work without internet
- **Depth**: AI can provide richer, more contextual information when available
- **Authoritativeness**: Definitions should feel trustworthy, not generated fluff

The system may use a hybrid approach — local data for speed on common words, with AI augmentation for depth and concept lookups.

---

## Settings

Minimal but important:

- **Hotkey configuration**: Change the summon shortcut
- **Word of the Day**: Enable/disable, set number of words
- **Default view**: Definition or Thesaurus first
- **AI usage**: Enable/disable AI-powered features
- **History retention**: How long to keep search history
- **Result type priority** (future): Drag-to-reorder the taxonomy to prioritize certain result types (e.g., Company before Word)

---

## Platform

WordLight is a desktop application for:

- **macOS** (primary)
- **Windows** (secondary)
- **Linux** (tertiary)

It should feel native on each platform — respecting system appearance (light/dark mode), keyboard conventions, and window management expectations.

---

## Success Metrics

1. **Time to answer**: How quickly can a user get the spelling or definition they need?
2. **Daily active usage**: Is this becoming a habit?
3. **Favorites growth**: Are users building vocabulary?
4. **Session frequency**: How often is WordLight summoned per day?

---

## Future Considerations

These are out of scope for v1 but worth noting:

- **Language support**: Definitions in other languages, translation
- **Pronunciation audio**: Hear the word spoken
- **Flashcard mode**: Study favorited words
- **Export**: Export favorites as a word list
- **Browser extension**: Access from web browsers
- **Integration**: Paste definitions directly, not just words

---

## Summary

WordLight is a fast, keyboard-driven word exploration tool and **vocabulary extender**. It helps users:

1. **Spell correctly** — with learning feedback that teaches, not just corrects
2. **Understand deeply** — OED-quality definitions with etymology and context
3. **Explore broadly** — look up people, companies, concepts, not just words
4. **Build vocabulary** — favorites, word of the day, and a discovery-focused empty state
5. **Stay in flow** — instant summon, keyboard navigation, invisible return to work

The interaction model follows Spotlight: summon → type → select → done. But unlike Spotlight, WordLight is designed for **word lovers** — people who don't just want answers, but want to learn and grow their language.

The interface is minimal, the information is authoritative, and the experience transforms quick lookups into opportunities for discovery.
