# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Toward 2.6.0, coverage scale: allophone rules for five more languages, phonotactic
profiles for all eleven core languages, the numeral follow-ons, and the citation re-check
carried over from 2.5.4. The decisions in it were the project owner's: re-sourcing both
tone rows, automatic variant aliases, Glagolitic's core values only, exact-environment
rules plus inventory fixes, and, for phonotactics, re-sourcing all twelve profiles with
each per-language choice listed below.

### Breaking

- **Inventories corrected to their sources** (every change is in *Fixed*). These
  phonemes are gone, so `phoneme_find` returns 0 for them:
  - Spanish `β ð ɣ`, and `"rr"`. The tap is now `"ɾ"` and the trill `"r"`, so `"r"`
    now names the trill, where it named the tap.
  - Japanese `t͡ɕ d͡ʑ ɸ ɕ ŋ`.
  - Mandarin's monophthongs `e o`.
- **Tone rows changed**: Thai `˦˩ → ˦˧` (falling) and `˨˩˦ → ˧˨˧` (rising), and every
  Vietnamese cell (below).
- **`char_value("Cyrl", …)` reads ѵ, ѿ and ҁ** (400, 800, 90) where it returned the
  unmapped sentinel.
- **20 scripts, not 19**, and 6 letter-value scripts, not 5 (Glagolitic). Anything that
  counts `script_all_codes` or `gematria_all_scripts` sees one more.
- **The English, Sanskrit and Japanese phonotactic profiles are re-sourced** (details in
  *Fixed*). What changes for a caller of `phonotactics_is_permitted`:
  - English: `"str" "spr" "skr"` are now spelled with the inventory's ɹ (`"stɹ"`), so
    the r forms answer unspecified (−1). `"stj"` is no longer listed (RP, not GA).
    `"sr"` is no longer forbidden, and reads unspecified. English now has 3 constraints,
    not 2, and their descriptions changed.
  - Sanskrit: the template is onset 3 / coda 1 / `(C)(C)(C)V(C)`, not 2 / 2 /
    `(C)(C)V(C)(C)`. The onset list (`pr kr tr sr pl kl`) is gone, and its one
    constraint is now the permitted final stops `k ʈ t̪ p`.
  - Japanese: the template is onset 2 / `(C)(C)V(C)`, not 1 / `(C)V(N)`. The coda list
    is `ɴ p t k s`. `"n"` is dropped: the moraic nasal is spelled ɴ, and /n/ is an onset.

### Added

- **allophone: rule sets for Spanish, Japanese, Russian, Mandarin and Arabic**, and
  `allophone_by_code` / `allophone_all_codes`. English was the only set. Each set comes
  from that language's JIPA Illustration of the IPA, and holds only the rules whose
  environment the model states exactly:

  | Set | Rules | From | Examples |
  |---|---|---|---|
  | `es` | 40 | Martínez-Celdrán et al. 2003, pp. 257–258 | nasal place (ánfora [ˈaɱfoɾa]), /s θ/ voicing before a voiced consonant, optional /s/ dentalization, /x/ → [χ], /l/ dentalization |
  | `ja` | 9 | Okada 1991, p. 95 | /tˢ s z/ → [tɕ ɕ ʑ] before /i/, /h/ → [ç ɸ], /z/ → [ᵈz], /g/ → [ɣ], palatalized /n/ |
  | `ru` | 16 | Yanushevskaya & Bunčić 2015, pp. 223–225 | final devoicing (11 pairs), vowel reduction to [ʌ] / [ə], /j/ → [i̯] |
  | `zh` | 2 | Lee & Zee 2003, p. 111 | [a] = [a̠] and [ə] → [ɚ] in open syllables |
  | `ar` | 17 | Thelwall & Sa'adeddin 1990, pp. 38–39 | /a aː/ → [ɑ ɑː] and /i/ → [ɨ] before the RTR consonants, /a/ → [ɐ] word-finally |

  "Before a voiced consonant" is written out as one rule per consonant, because a class
  such as Voiced is only a label: nothing evaluates it, and it would read as including
  vowels. The rules the model cannot state exactly are listed in each set's comment and
  not encoded. They include Spanish spirantization ("in other positions" than after a
  pause or nasal), Japanese /g/ → [ŋ] "medially", Russian voicing assimilation, Mandarin
  [a̟ ʊ] "in syllables closed by a nasal", and Arabic emphasis spread. Some rules the
  roadmap expected are not in these sources:
  - Okada treats /tˢ/ as a phoneme, so /t/ → [ts] is not his rule.
  - Yanushevskaya & Bunčić treat [ɨ] as a phoneme, not an allophone of /i/.
  - Lee & Zee never relate the apical vowel to /i/.

- **syllable: phonotactic profiles for Arabic, Mandarin, Hindi, Spanish, French, German,
  Russian, Korean and Portuguese**, and `phonotactics_by_code` / `phonotactics_all_codes`
  (12 codes: en ar zh hi ja es fr de ru ko pt sa). English, Sanskrit and Japanese were
  the only profiles. Each names one source, and its comment quotes the passage behind
  every value:

  | Profile | Template | From | What it encodes |
  |---|---|---|---|
  | `ar` | `CV(C)(C)` | Ryding 2005, §6, pp. 35–36 | onset required, no onset cluster (her /sk/ /br/) |
  | `zh` | `(C)V(C)` | Lee & Zee 2003, p. 111 | codas n ŋ only; m never final; ŋ never initial |
  | `hi` | `(C)(C)(C)V(C)(C)(C)` | Ohala 1983, ch. 4 | her word-initial clusters (66), native and homorganic nasal + stop finals, the Sanskrit-loan triple finals (34) |
  | `es` | `(C)(C)V(C)(C)` | Ríos Mestre 1999, §6.2 | obstruent + liquid onsets, not tl dl; C + s codas; no palatal coda |
  | `fr` | `(C)(C)V(C)` | Côté 2000; Fougeron & Smith 1993 | **partial, template unverified** (below) |
  | `de` | `(C)(C)(C)V(C)(C)(C)(C)(C)` | Schäfer 2018, §5.2 | his onsets and starred onsets; final devoicing; no [mf ŋf], nasal + nasal or nasal + liquid codas |
  | `ru` | `(C)(C)(C)(C)V(C)(C)(C)(C)` | Русская грамматика 1980, §22; Chew 2003, p. 86 | no [j] + consonant onset; consonant + [j] permitted |
  | `ko` | `(C)V(C)` | Lee 1993, JIPA, pp. 30–31 | the seven finals p t k m n ŋ ɾ |
  | `pt` | `(C)(C)V(C)` | Mateus & d'Andrade 1998 | **European Portuguese**: stop/f/v + liquid onsets; codas r l ʃ ʒ |

  Every sequence is spelled in its language's inventory symbols, and a test splits each
  one into that inventory's phonemes, so a sequence that can never match fails the
  suite (the v1.x English `"str"` and Sanskrit `"tr"` could not). A Permitted list is an
  enumeration its source prints, and a Forbidden list holds what it explicitly excludes.
  Anything else reads unspecified, which is not a ban. Each choice below was the
  project owner's:
  - **French is partial.** No reachable source states a French template (Walker 2001,
    Tranel 1987 and Dell 1995 could not be read), so `max_coda 1` is the view Côté
    (2000) reports and argues against, and French words do end in clusters (*arbre*).
    The onset list is Côté's minimum and misses vʁ (*vrai*).
  - **Portuguese is European.** The corpus inventory mixes in Brazilian /t͡ʃ d͡ʒ/; the
    roadmap carries that.
  - **Russian's onset is 4**, on Chew (2003): the Academy grammar says three, and
    взгляд begins /vzɡlʲ/.
  - **Hindi keeps long vowels** (`complex_nucleus 1`) to match the inventory, though
    Ohala treats Hindi vowels as contrasting in quality, not length.
  - **German counts the surface maximum**, onset 3 and coda 5 (*Herbsts*), not
    Schäfer's duplex core. His permitted codas are prose rules, so they are not listed.
  - **Korean's onset is 1**, after Lee (1993): glides are part of the diphthong, and the
    inventory has none to spell Sohn's (C)(G)V(C).
- **numerals: mispar gadol**, `NumericSystem.NUM_GADOL`: the Hebrew finals ך ם ן ף ץ
  count 500 / 600 / 700 / 800 / 900, and the other 22 letters keep their standard
  values (Jewish Encyclopedia, "Gematria", no. 11; Avodat Yisrael, Balak 2). The order
  is by value, not the mnemonic מנצפ״ך. Mispar gadol exists only for Hebrew, so on any
  other script it reports unmapped, and `script_alphabet_values` gives the shared empty
  table. The cache holds 6 scripts × 4 methods.
- **numerals: Glagolitic** (`gematria_glagolitic`, script `Glag`): 29 entries in
  Glagolitic's own order, 1–900 and chrivi 1000, with initial izhe as a second form of
  izhe. Every value is in each of Leskien 1922, Vajs 1932, Schenker 1995, Cubberley 1996
  and Everson & Cleminson 2003. The disputed ones are left out and listed: pe, yat as
  800, and the late thousands (sha 2000, yeru 3000, yu 5000).
- **numerals: the variant-glyph alias layer.** `char_value` and `string_value_in` read
  izhitsa ѵ as у (400, the Old Church Slavonic form), ot ѿ as ѡ (800, the Synodal form)
  and koppa ҁ as ч (90, the earlier ustav period). The canonical tables are unchanged,
  as architecture note 001 requires. Monograph uk ꙋ is not aliased: no source shows it
  used as a numeral, which also corrects the Cyrillic table's own note.
- **script: Glagolitic** (`script_glagolitic`), registered because its numerals are:
  every letter-value script must be a registered script. The ranges are Unicode's
  Glagolitic and Glagolitic Supplement blocks. Its attestation dates are left unset,
  because no source for them was checked.

### Fixed

- **Thai tones re-sourced: the third Wikipedia value credited to a primary source.**
  The 2.4.1 row, 33 / 21 / 41 / 45 / 214, was credited to Tingsabadh & Abramson (1993),
  who print no pitch numbers (p. 26 gives diacritics only). It matches the uncited
  "Phonetic" column of Wikipedia's *Thai language* table. The row is now Teeranon (2007,
  p. 1), after Abramson's acoustic work: `˧ / ˨˩ / ˦˧ / ˦˥ / ˧˨˧`. No source checked
  prints 41 or 214 for Thai.
- **Vietnamese tones re-sourced to Kirby (2011)**, the JIPA Illustration, p. 386, read
  from the rendered tone letters: `˦ / ˧˨ / ˨˦ / ˧˩˨ / ˧˨˥ˀ / ˨ˀ`. The ˀ marks are for his
  "broken" and "low glottalized". In the old row, ngang 33, huyền 21 and sắc 35 were
  Nguyễn & Edmondson's. But hỏi 313, ngã 3ˀ5 and nặng 21ˀ, credited to them and to
  Thompson, match the uncited Northern column of Wikipedia's *Vietnamese phonology*
  table instead.
- **Spanish inventory**, after Martínez-Celdrán et al. (2003): [β ð ɣ] are allophones of
  /b d ɡ/ ("their approximant variants", p. 257), so they are removed. The rhotics are
  now /ɾ/ (tap) and /r/ (trill); before, "r" was the tap and "rr" the trill. /ɟ͡ʝ/ is
  added.
- **Japanese inventory**, after Okada (1991): [t͡ɕ d͡ʑ ɸ ɕ ŋ] are allophones, now rules,
  so they are removed, and the moraic nasal /ɴ/ is added. Consonants 20 → 16.
- **Mandarin inventory**, after Lee & Zee (2003):
  - /ŋ w j/ are added.
  - The monophthongs are i y a ə u ɤ, so /ə/ is added and /e o/ removed; those two
    occur only in the diphthongs.
  - The syllabic apical [ɹ̩] is added.
  - Their 11 diphthongs and 4 triphthongs are added as units.

  The retroflex symbols stay, a Chao (1968) convention Lee & Zee's own notes record.
- **Arabic inventory**: the marginal pharyngealized /lˤ/ is added (Thelwall &
  Sa'adeddin 1990, p. 37).
- **Russian inventory** matches Yanushevskaya & Bunčić (2015) and is unchanged.
- **English phonotactics re-sourced to Gimson** (Cruttenden 1994, §10.9.1), adjusted to
  General American, which the inventory is. v1.x wrote r for the inventory's ɹ, so its
  `"str" "spr" "skr"` never matched a word built from the inventory. It described all
  eleven as "/s/+stop", lacked /skl/ (*sclerosis*), and banned /sr/, which Gimson lists
  in loans (*Sri Lanka*). The CCC onsets are now Gimson's /s/ + voiceless stop +
  /l ɹ j w/, without the RP /stj/ (*stew*). The template, 3 / 4, was already his.
- **Sanskrit phonotactics re-sourced to Allen (1953)**: one final consonant in pausa
  (Whitney 1889, §150, agrees), the non-nasal finals k ʈ t̪ p (p. 70), and an onset of
  three (*strī*, Macdonell 1927, §100a). v1.x allowed two-consonant codas, and its
  onset list had no source and spelled t for the inventory's t̪.
- **Japanese phonotactics re-sourced to Kubozono (2015)**: "(Cj)V(VC)", so Cj onsets
  (*kja*) and the moraic obstruent coda (*it.tai*), which v1.x's `(C)V(N)` excluded.
- **WALS claims checked against WALS's own data** (CLDF): Mandarin 85A "No dominant
  order", Hindi 98A "Tripartite", and Somali and Hausa 13A "Simple tone system". All
  four hold.

### Performance

Measured interleaved against a build of 2.5.4, min of 3, timer floor 1.21–1.24 µs:

- **`gematria_char_value`, 292 → 307 ns (+5%).** This is the cost of the alias and
  mispar-gadol dispatch that every lookup now passes through. A cached call still
  allocates nothing: a test measures 1,000 calls through each path.
- **`gematria_alphabet_table` is unchanged** (214 → 219 ns, inside its spread). It
  first read +20.7%: the script-index loop did not stop at a match, so the sixth script
  cost one more string compare on every call. It now stops at the match.
- `grammar_by_code_lookup` and `lexicon_find_word` read −8% with their code unchanged.
  That is the per-run CPU effect 2.5.4 measured, not a result.

The phonotactics rows, measured interleaved against the committed allophone work (min of
3 runs; every row with unchanged code and data stayed within ±5.3%, apart from the three
single-digit-nanosecond inventory rows at the timer floor):

- **`json_phonotactics_write_en`, 5,509 → 8,549 ns (+55%), and `json_phonotactics_read_en`,
  16,524 → 21,955 ns (+33%).** The code is unchanged. The English profile grew from 2
  constraints and 15 sequences to 3 and 19, with longer descriptions, so its document grew
  from 425 to 592 bytes (+39%). Past 512 bytes the string builder (64-byte start, doubling)
  grows once more, which is part of the write's extra cost; how much was not measured.
- **`phonotactics_is_permitted`, 143 → 163 ns (+14%)**, also with unchanged code. The row
  looked up `"str"`, which is no longer in the list and would now time a miss across all
  three constraints, so it looks up `"stɹ"`. That is the fourth sequence of the first
  constraint, where `"str"` was the fourth, but its ɹ is two bytes. The two rows measure
  different strings, so the numbers are not a like-for-like comparison.

## [2.5.4] - 2026-11-13

Roadmap 2.5.4, hygiene and follow-ons: a cached numerals table, test-only deps,
logging through sakshi, Thai vowel length, Yoruba nasal vowels, a nasal-vowel defect
in Portuguese and Guarani, and an investigation of why benchmark rows move. Three items
carry over (see the roadmap): the bayan scan waits on a bayan release, filed there as
an issue; Osatananda's Lao values, whose source is unreachable; and the full
citation re-check.

### Breaking

- **Portuguese and Guarani nasal vowels are respelled** with the combining tilde
  (U+0303): `"ã"` (U+00E3) is now `"a"` + U+0303, and likewise ẽ ĩ õ ũ. A caller passing
  the precomposed form, which NFC text produces, must decompose it (NFD) first, or
  `phoneme_find(pt, "ã")` returns 0. The same holds for six Swadesh transcriptions
  (Portuguese, and Hindi नहीं), whose IPA was respelled to match.
- **`script_alphabet_values` returns a shared table** (see *Changed*).

### Changed

- **numerals: `script_alphabet_values` is cached.** It was the only table builder in
  `numerals.cyr` without a cache. Every call built a fresh table of about 856 B, and the
  bump allocator never frees, so a caller in a loop grew the heap without bound. Now
  the first call for a (script, method) pair builds the table, and every later call
  returns the same vec and pair records. A cached call allocates nothing; the
  uncached version, measured, allocated 840,000 B over 1,000 calls.

  **Aliasing, the one semantic change:** the result is now shared and read-only. A
  caller that writes into a pair, or pushes onto the vec, changes it for every later
  caller. Copy it first if you need to edit it. Every unknown script gets the same
  shared empty vec. A method other than ordinal or reduced still reads as standard,
  as before, and now shares the standard table.

- **`assert` and `bench` moved to `[test] stdlib`** (cyrius 6.7.6+), so they are no
  longer prepended to the library build. No file under `src/` calls either. They
  are still vendored and locked. Measured before → after, without sakshi (added
  below):

  | Build | Before | After |
  |---|---|---|
  | default | 551,248 B | 533,984 B (−17,264) |
  | `-D LOGGING -D MCP -D DAIMON -D HOOSH` | 579,080 B | 561,816 B (−17,264) |
  | default, `CYRIUS_DCE=1` | 63,824 B | 62,944 B (−880) |
  | full, `CYRIUS_DCE=1` | 67,080 B | 66,200 B (−880) |

  A test file now builds through `cyrius test`, not a bare `cyrius build`, which no
  longer prepends `assert`.

- **`-D LOGGING` goes through sakshi**, the AGNOS tracing library, instead of a bare
  stderr writer. `"sakshi"` is declared in `[deps].stdlib`, as the manifest's own note
  for `-D LOGGING` said, and locked (36 deps). sakshi timestamps and levels each event,
  and `sakshi_set_output` can send it to a file, a ring buffer, UDP or a hook: the audit
  trail CLAUDE.md calls for. Its default target is stderr, as before.

  **`VARNA_LOG`, as in v1.x.** v1.x built its filter with
  `EnvFilter::try_from_env("VARNA_LOG").unwrap_or_else(|_| EnvFilter::new(default))`.
  `varna_log_init_with_level(level)` now does the same: a valid `VARNA_LOG` (`trace`,
  `debug`, `info`, `warn`, `error`, `off`) wins over the caller's level, and an absent
  or invalid one falls back to it. A tracing directive such as `varna=debug` is not
  parsed, so it falls back. `off` is new: it silences everything. The API is
  unchanged: `varna_log_init`, `varna_log_init_with_level`, `varna_log_level`,
  `varna_log`.

  **Cost:** sakshi is prepended to every varna build, logging or not. The default
  binary grows 533,984 → 563,760 B (+29.8 KB), or by 9.3 KB under `CYRIUS_DCE=1`.
  `dist/*.deps` does not list sakshi, so a consumer that folds `dist/varna.cyr` with
  `-D LOGGING` declares `"sakshi"` itself.

### Fixed

- **Thai has vowel length and its three diphthongs.** The block had the nine
  qualities only, though the corpus already cited Tingsabadh & Abramson (1993), the
  JIPA Illustration, for Thai's tones. Their p. 25: "There are nine vowels. Length is
  distinctive for all the vowels." They write long vowels with ː. The three
  diphthongs are /ia ɯa ua/. Thai now has 21 vowels (9 + 9 + 3). Short and long
  vowels share a quality, because the authors found the spectral differences "too
  subtle to place with confidence". The 21 consonants already matched the source.

- **Yoruba has nasal vowels: /ĩ ũ ɔ̃/.** This is the three-vowel analysis of the
  source Yoruba's ATR values already come from: "nasal vowel counterparts for the
  high and low vowels only" (Archangeli & Pulleyblank 1989, p. 175, n. 5).
  Pulleyblank (in Comrie, p. 972) prefers the same "three-way contrast", and has ɔ̃
  and ɑ̃ as "variants of a single phoneme". The descriptive grammars' marginal fourth,
  ɛ̃ (Bamgboṣe; Capo 1985), is left out, and the source comment says so. No source
  states a nasal vowel's ATR value, so these are inferred from the oral vowels: ĩ ũ
  +ATR, ɔ̃ −ATR. They are spelled with the combining tilde (U+0303), the corpus
  convention, so the precomposed U+0129 that NFC text produces does not find /ĩ/.

- **Portuguese's five nasal vowels and five of Guarani's six read as [−nasal].** They
  were written with the precomposed letters (ã ẽ ĩ õ ũ) and carried no `Nasalized`
  bit, so `phoneme_df(p, DF_NASAL)` answered − for all ten. The vowel test could not see
  it, because it checks the combining tilde. Only Guarani ɨ̃, written with that tilde,
  was right. All ten now carry the bit and the corpus's one spelling (*Breaking*). A
  new test fails on any precomposed nasal vowel in an inventory or a Swadesh
  transcription. A mutation run that put one ã back failed 3 assertions.

- **Tests.** Corpus totals moved by exactly these changes: long vowels 118 → 127,
  nasal vowels 5 → 18 (Yoruba 3, Portuguese 5, Guarani 5), languages with a marked
  vowel 23 → 26, ATR-specified vowels 22 → 25. Every new or corrected vowel is pinned.

### Performance

- **`gematria_alphabet_table`: 1,291 → 211 ns (−83.7%)**, from the cache, as min of 3
  runs interleaved against a 2.5.3 build (timer floor 1.21–1.24 µs). A second
  session gave 1,279 → 201 ns. What remains is finding the script's slot. A cached
  call allocates nothing.

- **Why benchmark rows move with no code change** (the 2.5.4 investigation).
  `grammar_by_code_lookup` and `lexicon_find_word` had moved 10–13% across builds
  that never touched them. Each candidate was tested in a scratch tree, with runs
  interleaved:

  | Candidate | Test | Result |
  |---|---|---|
  | Function-entry alignment | cycc does not align entries (`&lexicon_find` sat at offset 63 mod 64). 16 builds moved the hot functions through offsets 0–63, and 16 more moved only the benchmark wrappers, 2 rounds each | No effect. Rows stayed within 216–232 ns and 765–819 ns, unrelated to offset; the slow rows in one round were fast in the other |
  | Where the harness sits | 2.5.4 with `assert` and `bench` back in `[deps].stdlib`, against 2.5.4 as shipped | No effect: 218 vs 217 ns |
  | sakshi, ~30 KB ahead of varna's code | 2.5.4 with and without it | ≤ 1.8%, inside the spread |
  | The run itself | One fixed binary, run 8 times; then pinned with `taskset` | 226–238 ns in two clusters; cpu0 and cpu2 ~227 ns, cpu4 ~237 ns |

  The host is a Ryzen 7 5800H on the `powersave` governor, so clock speed varies by
  core and over time. That per-run state is the effect that reproduces. It is not
  small: 2.5.4 against 2.5.3, interleaved, read +7.5% on these untouched rows in one
  session (spread 2–3%) and +2.4% in the next (spread 5.5%). So a move under about 10%
  on code that did not change is not a result until it reproduces across sessions.
  Nothing was filed with cyrius, because the roadmap's condition was "if alignment
  is the cause", and it is not. Pinning a core and a fixed governor for benchmark
  runs would narrow the spread; `scripts/bench-history.sh` does neither yet.

## [2.5.3] - 2026-11-13

Roadmap 2.5.3, data accuracy. Glottocodes for all 51 languages, the Burmese, Lao
and Somali items, each re-sourced to named descriptions, and the tongue-root ([ATR])
feature, set for the Yoruba and Wolof vowels. The Burmese, Lao and Somali changes were
researched, then put through an adversarial review that tried to refute them, as at
2.4.1. The ATR values skipped that review, because no source disagreed on any vowel.
Then every quote and value used here was checked against the primary text. That last
step found that **two of the corpus's existing citations were Wikipedia's text
credited to a primary source**, and that a third over-claimed its sources (see
*Fixed*).

### Breaking

Data corrections that a consumer can observe:

- **Lao has 5 tones, not 6.** A caller indexing `phoneme_tones(lo)` at 5 now reads past
  the end, and slots 1–4 hold different tones (Enfield's order, below). Find a tone by
  shape (`tone_contour`, `tone_start_level`) rather than by a position taken from 2.5.2.
- **Removed segments:** Lao `/t͡ɕʰ/` and Somali `/ɣ/`. `phoneme_find` returns 0 for
  both. Neither exists in the language (below).
- **Burmese tone strings changed** (`˥ ˧ˀ ˨˩ ˥˧ʔ` → `˦ ˥˩ˀ ˨ ˥˩ʔ`), and the checked
  tone's derived register is now `TONE_MID` rather than `TONE_HIGH`. Its high onset is
  `tone_start_level`, 5.
- **Yoruba and Wolof vowels carry feature bits**, so `phoneme_features(p)` is no longer
  0 for them, and their JSON `features` arrays now hold `"AdvancedTongueRoot"` or
  `"RetractedTongueRoot"`. A 2.5.2 reader refuses those names, as it refuses any
  flag it does not know. 2.5.2 documents still read in 2.5.3.

### Added

- **registry: a Glottocode on every entry** (roadmap 2.5.3, deferred from 2.3.0).
  `registry_info_glottocode(li)` returns the Glottolog identifier (`"stan1293"` for
  English), or 0 when an entry has none. `registry_by_glottocode(gc)` goes the other
  way, for a caller holding a Glottolog or PHOIBLE identifier, and
  `registry_glottocode_well_formed(s)` checks a string's shape (four lowercase letters
  or digits, then four digits).

  All 51 came from Glottolog itself on 2026-11-13: each entry's ISO 639-3 code went
  through Glottolog's resolver (`glottolog.org/resource/languoid/iso/<iso>`), and the
  name it returned was checked against the entry. None was written from memory. A
  Glottocode is an opaque identifier, so a remembered one is a plausible fabrication,
  not an approximation.

  Each is the **language-level** languoid for the variety the inventory transcribes.
  For a macrolanguage code, that is the standard individual language: `sw` → swh,
  `ar` → arb, `zh` → cmn, `fa` → pes, `gn` → gug and `mn` → khk; `nah` is nci, as its
  scope comment already said. Two entries needed a judgement, and both are pinned with
  the reason:

  | Entry | Glottocode | Why not the obvious one |
  |---|---|---|
  | `grc` Koine Greek | `anci1242` | Glottolog's `koin1234` "Koineic Greek" is the clade of Koine's *descendants* (Medieval, Modern and Pontic Greek), not Hellenistic Koine. |
  | `qu` Southern Quechua | `cusc1236` | Southern Quechua, `quec1389`, is a subgroup, and it is already this entry's genus. The inventory's plain / aspirated / ejective stops are Cusco-Collao, which Ayacucho lacks. |

- **tests/glottocode.tcyr**, 127 assertions. All 51 values are pinned one by one, each
  with the name Glottolog returned, because a sweep cannot see two rows swap codes. It
  also covers the reverse lookup, presence, shape and uniqueness across the corpus, and
  the edge cases of the shape check. A mutation run swapping `fr` and `de` failed 4
  assertions; the unmutated control passed.

- **phoneme: the tongue-root feature, [ATR]**, in two bits:
  `PhonemeFeature.AdvancedTongueRoot` (16384, [+ATR]) and
  `PhonemeFeature.RetractedTongueRoot` (32768, [−ATR]). Two bits rather than one, so
  that a vowel can be +, − or unspecified. One bit could not tell Yoruba's −ATR /ɛ/ from
  English /ɛ/, which has no ATR value at all. The bits are not written into the IPA
  string with ̘ / ̙. Both languages already spell the contrast with distinct letters
  (e ~ ɛ, o ~ ɔ, ə ~ a), so `phoneme_find(yo, "e")` is unchanged.

  `features.cyr` derives `DF_TENSE_ATR` from the bits alone: + for
  `AdvancedTongueRoot`, − for `RetractedTongueRoot`, unspecified for neither. It never
  guesses from the letter. PHOIBLE's symbol-derived [tense] does guess from the letter,
  and in an ATR system that misplaces the very vowels the harmony is about: Wolof /ə/ is
  +ATR and /a/ −ATR.

  The values were researched per vowel and checked against the primary tables. Every
  source agrees, so no vowel is left unspecified:

  | | +ATR | −ATR | Source |
  |---|---|---|---|
  | Yoruba (7) | i e o u | ɛ a ɔ | Archangeli & Pulleyblank (1989), *LI* 20(2), (1a) p. 174; Przezdziecki (2005) Table 2.1 |
  | Wolof (15) | i u e o ə, and iː uː eː oː | ɛ ɔ a, and ɛː ɔː aː | Unseth (2009) Table 1; Sy (2006), *SAL* 35(2), ex. (1) p. 204 |

  The high vowels are + in both languages. That is the value every source assigning
  one gives them, and in Wolof they trigger harmony word-initially ("All vowels trigger
  harmony in initial position", Sy p. 204). Archangeli & Pulleyblank leave Yoruba
  i, u and a unspecified in their underlying forms. The corpus records the surface
  values, as their (1a) does. Whether the articulator really is the tongue root is
  less settled, and the source comments say so: Archangeli & Pulleyblank knew of no
  phonetic study showing it for Yoruba (p. 173, n. 2), and Dye's (2015) ultrasound of
  Wolof found only a small /e/–/ɛ/ difference. The JSON reader refuses a vowel with
  both bits.

  Somali has a second vowel series too, and stays unmarked on purpose (see *Fixed*).

### Changed

- **`LanguageInfo` is 88 bytes**, with the Glottocode at `[80]`. The private
  constructor `_li_new_geo` takes it as a fourth argument, after the script code. The
  compiler checks arity, so any external caller of that private name fails to build
  rather than misreading its arguments.

- **Registry JSON gains `"glottocode"`**, appended after `longitude_mdeg` so no 2.5.2
  field moves. It is written as a string or `null`. On read it may also be absent,
  because every 2.5.2 document lacks it, and such a document still decodes, with
  glottocode 0. If present, it must be a well-formed Glottocode string or `null`;
  anything else (`7`, `""`, `"STAN1293"`, `"abcd123"`) refuses the entry, as a wrong
  type does for every other field.

### Fixed

- **Burmese tones re-sourced to one description**, Watkins (2001), "Burmese", the JIPA
  Illustration, p. 293: high `˦`, creaky `˥˩ˀ`, low `˨`, checked `˥˩ʔ` (was `˥ ˧ˀ ˨˩
  ˥˧ʔ`). Watkins sets the tone letters in a custom font, so the page was rendered and
  the strokes measured against the bar: low 2.1, high 3.9, and creaky and checked
  5.0 → 1.6 on one shared glyph. Creaky and checked now share a pitch shape, as every
  description has them. Before, creaky sat two levels below checked.

  The 2.4.1 row's one "sourced" cell was not sourced. Its "Wheatley (1987)" values,
  `˥˥˦ / ˥˧ˀ / ˧˧˦ / ˥˧ʔ`, are the uncited IPA column of Wikipedia's *Burmese language*
  tone table. Wheatley's footnote there sits on the prose column, which has no Chao
  letters. 2.4.1 also took `˥˧ʔ` over Watkins' `˥˩ʔ` only so that the derived register
  would come out high. That bent a source's value to fit a derivation rule, which is
  now undone. `˥˩` derives `TONE_MID` because register is the rounded mean of the
  levels, the same as Mandarin's tone 4 already. Switching to an onset-based rule was
  considered and rejected for this release: it would move 11 of the corpus's 37 tones
  across five languages, and Mandarin's rising tone 2 `˧˥` would become mid.

- **Lao rebuilt from one grammar**, Enfield (2007), *A Grammar of Lao*, ch. 3. The 2.5.2
  block, unchanged since v1.x, was not a reduced transcription. It was a fragment:

  | | 2.5.2 | 2.5.3 |
  |---|---|---|
  | Tones | 6: `˧ ˨˩ ˥˩ ˦˥ ˩˧˥ ˧˥`, labelled in v1.x with Thai tone names | 5: `˧ ˧˦ ˨˩˥ ˦˥˩ ˧˩` |
  | Consonants | 20, including Thai's `/t͡ɕʰ/`, with no `/ɲ/` | 20: `/t͡ɕʰ/` removed (Lao ຊ is /s/), `/ɲ/` (ຍ) added |
  | Vowels | `i ɯ u e` + long, and `ia`: **no `/a/`** | 9 qualities × length (18), and `ia ua ɯa` |

  Enfield: "There are five distinct lexical tones in the variety of Lao spoken by
  natives of Vientiane" (p. 35). Osatananda's 1997 dissertation on Vientiane tone says
  five in its abstract, and Ueda (1994) has five too. The old row had two high rises
  (`˦˥` and `˧˥`), and no source prints its `˩˧˥`. Enfield prints no pitch numbers, so
  the values are Yanagimura (2011), *Journal of Mekong Societies* 7(1), Table 1. Those
  are isolation forms. Yanagimura numbers the tones differently, so his were matched to
  Enfield's by example word. In Enfield's order, with Enfield's examples: 33 (khaa1
  'galangal'), 34 (khaa2 'to be stuck'), 215 (khaa3 'leg'), 451 (khaa4 'commerce'),
  31 (khaa5 'slave'). Yanagimura's own prose calls 451
  "high rising-falling" and 215 "low falling-rising", so they parse as peaking and
  dipping, as Thai's citation-form `˨˩˦` already does. The 2.4.1 review was right that
  the low rise of ຂາ 'leg' must stay. It is tone 3, and it has the lowest onset.

  Osatananda's own values (reported by Wikipedia as 33 / 35 / 13 / 52 / 31) could not
  be read and are not used. The review changed two symbols from the first proposal.
  `/w/` stays: Enfield writes initial v- (labio-dental) and final -w as one phoneme, so
  ʋ would misdescribe the coda. The mid unrounded vowel is `ɤ` beside `ɯ`, as in the
  Thai row, where Enfield writes ə. All four of PHOIBLE's Lao inventories have `/ɲ/`,
  and none has `/t͡ɕʰ/`.

- **Somali consonants.** Each change survived the review, and each was checked against
  the source it cites. Values the sources dispute stay, with the dispute noted:

  | Segment | Change | Evidence |
  |---|---|---|
  | `/t͡ʃ/` ⟨j⟩ | added | `t̠ʃ` in both PHOIBLE inventories; Edmondson, Esling & Harris (2004): "voiceless aspirated palato-alveolar affricate", voiced [dʒ] in southern varieties |
  | `/ɣ/` | removed | `/ɡ/` between vowels: Gabbard (2010) derives "[γ] from /g/"; neither PHOIBLE inventory lists it |
  | `/t d/` | Alveolar → Dental | Edmondson et al. "apico-dental"; PHOIBLE 552 `t̪ʰ d̪` |
  | `/q/` | kept voiceless | Gabbard's spectrograms show "never any voicing or aspiration"; Saeed (1999) and PHOIBLE write `/ɢ/` |
  | `/x/` | kept | SPA and both PHOIBLE inventories; marginal, mostly Arabic loans |

- **Somali's vowels: unchanged, but they now cite the right source.** From 2.4.1 the
  corpus said the five qualities were "Orwin (1994)", whose "own wording" was that "each
  of these five vowels has a fronted (ATR) variant". That sentence is Wikipedia's
  paraphrase. Orwin's thesis (*Aspects of Somali Phonology*, p. 61) sets out "two series
  of vowels". The five-vowel listing is his textbook, *Colloquial Somali* (1995), which
  leaves fronting aside. Five qualities plus length stay, after Kimper, Bennett, Green &
  Yu (2019, p. 199): Somali is "commonly described as containing five major vowel
  categories {i,e,a,o,u}, each of which is contrastive for length and (purportedly)
  for an additional feature". That feature is described in the source comment rather
  than marked per vowel. Sources disagree on what it is (front/back, ±ATR, or a voice
  register), the ten-symbol listings conflict, and Kimper et al. found "no clear
  evidence" of a categorical contrast.

  That settled what gated the `AdvancedTongueRoot` bit since 2.4.1. Somali does not
  carry it, and the comment says why. The bit landed for Yoruba and Wolof (*Added*).

- **Wolof's vowel citation.** From 2.4.1 the comment said the vowels were "Ka (1994)
  as presented in Unseth (2009)" and that "the same assignment appears in Torrence
  (2013) and PHOIBLE". Unseth's paper is a student paper. It bases its data on Ka
  (1993, not 1994), but does not credit its tables to him. Torrence could not be checked.
  And PHOIBLE marks no ATR value on any Wolof vowel: it encodes e ~ ɛ as [tense], taken
  from the letter. The comment now cites Unseth and Sy directly. Its ATR values were
  right and are unchanged.

- **Tests** pin all five Lao tones and all four Burmese tones by index, plus the Lao and
  Somali corrections segment by segment. The ATR group in `tests/vowel_features.tcyr`,
  which pinned the bit's absence, now pins all 22 Yoruba and Wolof vowels one by one.
  It also checks that no consonant, no Somali vowel and no IPA string carries an ATR
  mark. A mutation run flipping Wolof /ə/ to −ATR, which leaves every count unchanged,
  failed in both `vowel_features` and `distinctive`; the controls passed. Two corpus counts moved by exactly these
  changes: aspirated consonants 76 → 75 (Lao `/t͡ɕʰ/`), and long vowels 113 → 118 (Lao).
  The corpus now has 36 tones (was 37).

### Performance

- **Registry JSON is 9–12% slower to write and read**, because English's document
  grew by the 25 bytes of `,"glottocode":"stan1293"`, about 12%. Each column is a
  separate session interleaved against a build of 2.5.2 in the same boot, reporting
  the minimum (timer floor 1.21–1.33 µs):

  | Row | Glottocodes only (min of 2) | Final tree (min of 3) |
  |---|---|---|
  | `json_registry_write_en` | 2,167 → 2,431 ns (+12.2%) | 2,343 → 2,545 ns (+8.6%) |
  | `json_registry_read_en` | 7,423 → 8,278 ns (+11.5%) | 7,952 → 8,863 ns (+11.5%) |

  In the final run, one other row moved by more than its own spread:
  `swadesh_by_code_lookup`, 108 → 99 ns (−8.3%, spread under 5%), with its code
  unchanged. That is the code-layout effect already filed in 2.5.4.
  `lexicon_find_word`, +5.0% in an intermediate run, was back within noise. The data
  and ATR changes are not on any benchmarked path: no benchmark covers Burmese, Lao,
  Somali, Yoruba or Wolof. `phoneme_df` gains two bit tests on the vowel path only.

## [2.5.2] - 2026-11-05

Roadmap 2.5.2 (P0), bayan / serde integration. **Every type v1.x serialized has JSON
again**, in v1.x's format, and all 26 of `serde_roundtrip.rs`'s serde tests are ported.
The port's dropped variants are back, the `-D` surfaces sit on the same serializers, and
`dist/varna-json.cyr` is how a consumer opts in.

### Breaking

- **`-D MCP`: `varna_phonemes`, `varna_script` and `varna_grammar` return the whole
  record**, as v1.x's `serde_json::to_value` did. The port had returned a few hand-picked
  fields, and `script_type` as a bare integer.

  | Tool | Was | Now |
  |---|---|---|
  | `varna_phonemes` | `language_code`, `language_name`, `consonant_count`, `vowel_count` | the inventory: `phoneme_inventory_to_json` |
  | `varna_script` | `code`, `name`, `script_type` (an integer) | the script: `script_to_json` |
  | `varna_grammar` | `language_code`, `case_count`, `gender_count` | the profile: `grammar_profile_to_json` |

  **Migration**: fields kept their names, so `language_code`, `case_count` and the like
  read as before. `script_type` is now a name, `"Alphabet"`, not `0`. `consonant_count`
  and `vowel_count` are gone, as they were not in v1.x: count `phonemes` by the key
  inside `kind`, or call `phoneme_consonant_count`. `varna_translate_ipa` and
  `varna_compare` keep their shape.

### Added

- **JSON for phonemes and inventories, in the v1.x serde format** (new `src/json.cyr`,
  [ADR 0003](docs/adr/0003-json-wire-format-matches-v1x-serde.md)):
  `phoneme_to_json(p, sb)` / `phoneme_from_json(text)` and
  `phoneme_inventory_to_json(inv, sb)` / `phoneme_inventory_from_json(text)`. The
  output is serde's default representation of the v1.x types, field for field, with
  every enum value by name. The two fields varna gained in 2.2.x go inside the variant
  object:

  ```json
  {"ipa":"pʰ","kind":{"Consonant":{"manner":"Plosive","place":"Bilabial","voiced":false,"airstream":"Pulmonic","features":["Aspirated"]}}}
  ```

  An inventory is `{"language_code":..,"language_name":..,"phonemes":[..],"tones":null|[..],"stress":"Free"}`,
  in v1.x's field order. Tones are written as their Chao strings (`tone_letters`), as
  v1.x wrote them, and re-parsed on read.

  Readers are as strict as serde's defaults and return 0 for any of: a missing
  required field, a wrong JSON type (`"manner":0`, `"voiced":1`), an unknown name, an
  externally tagged object with other than one key, a duplicate key, or a vowel
  claiming a non-pulmonic airstream. Unknown extra keys are ignored, as serde ignores
  them. `airstream` and `features` are optional on read, so a **v1.x document
  decodes** (as Pulmonic, no features). A writer refuses a null record or a corrupt
  `kind` by returning -1, and writes nothing.

- **JSON for the phonology types**, the same way: `allophone_rule_to_json` /
  `_from_json`, `allophone_rule_set_to_json` / `_from_json`,
  `syllable_template_to_json` / `_from_json`, `phonotactic_constraint_to_json` /
  `_from_json`, and `phonotactics_to_json` / `_from_json`. Each matches v1.x's serde
  output field for field:

  ```json
  {"phoneme":"t","allophone":"ɾ","environment":"Intervocalic","obligatory":false}
  {"max_onset":3,"max_coda":4,"complex_nucleus":true,"pattern":"(C)(C)(C)V(C)(C)(C)(C)"}
  {"kind":"Permitted","position":"Onset","sequences":["str","spr"],"description":".."}
  ```

  A rule set is `{"language_code":..,"rules":[..]}`, and a profile is
  `{"language_code":..,"syllable":{..},"constraints":[..]}`. The `u8` fields
  (`max_onset`, `max_coda`) must be JSON integers in 0..255, as serde requires.

- **`allophone`: v1.x's `Before(PhonemeClass)` / `After(PhonemeClass)` environments**,
  which the port deferred. `EnvBefore(cls)` and `EnvAfter(cls)` build one, and
  `PhonemeClass` holds v1.x's seven classes plus `ClassSpecific(ipa)`:

  | v1.x | varna | JSON |
  |---|---|---|
  | `Environment::Intervocalic` | `Environment.Intervocalic` | `"Intervocalic"` |
  | `Before(PhonemeClass::Nasal)` | `EnvBefore(PhonemeClass.ClassNasal)` | `{"Before":"Nasal"}` |
  | `After(PhonemeClass::Specific("k".into()))` | `EnvAfter(ClassSpecific("k"))` | `{"After":{"Specific":"k"}}` |

  The constants differ from v1.x's names because Cyrius enum constants are global, so a
  second `Vowel` is "duplicate symbol ... last definition wins". `Environment` itself
  is unchanged, since `#derive` refuses an enum with a payload variant; these two
  variants are a separate `ClassEnvironment`, stored as a pointer in the rule's
  environment slot. A unit environment is still a plain int, and every existing caller
  works as it did. New `environment_eq`, `environment_is_conditioned`,
  `environment_side`, `environment_class`, `phoneme_class_eq`,
  `phoneme_class_is_specific` and `phoneme_class_ipa`. `allophone_realize` now compares
  by value, as v1.x's `==` did, so a caller can build the environment it asks about. As
  in v1.x these are labels matched by equality: nothing evaluates a class against a
  phoneme, and `allophone_english()` uses neither. Rule writers now also refuse a
  corrupt environment (-1, nothing written); one out of range used to be written as
  `null`.

- **JSON for the script module**, in the v1.x format: `script_to_json` / `_from_json`,
  `numeral_mapping_to_json` / `_from_json`, `numeral_system_to_json` / `_from_json`, and
  `transliteration_table_to_json` / `_from_json`:

  ```json
  {"code":"Latn","name":"Latin","script_type":"Alphabet","direction":"LeftToRight","status":"Living","attestation":null,"unicode_ranges":[[65,90],[97,122],[192,255],[256,591]],"languages":["en","fr","es","de","pt","it"]}
  {"scheme":"IAST","source_script":"Deva","target_script":"Latn","forward":[["अ","a"],["आ","ā"]]}
  {"character":"ς","value":200,"role":"ROLE_ALLOGRAPH","base":"σ"}
  ```

  A tuple is a two-element array, and `u32` fields are JSON integers in 0..4294967295.
  `role` and `base` are additive (2.4.0): a v1.x mapping, `{"character":"α","value":1}`,
  decodes as a letter with no base. The reader holds the constructors' invariant, so an
  allograph without a base and a base on anything else are both refused.

- **JSON for grammar, lexicon, cognate and dialect**, in the v1.x format:
  `grammar_profile_*`, `lex_entry_*`, `lexicon_*`, `cognate_entry_*`, `cognate_set_*`,
  `etymology_*` and `language_variety_*`, each `_to_json` / `_from_json`. A Swadesh list
  is a `Lexicon`, so it needs nothing of its own.

  ```json
  {"word":"water","ipa":"ˈwɔːtər","gloss":"water","pos":"Noun","frequency_rank":250,"swadesh_index":23}
  {"source_language":"fr","source_form":"café","borrowing_type":"Loanword","period":null}
  ```

  A grammar profile writes v1.x's eight fields, then the nine from 2.3.0, which are
  optional on read and take `_gp_new`'s defaults. So a v1.x document decodes to a profile
  that states it does not carry them. `has_future` is a tri-state, so it is an
  `Option<bool>` on the wire: `true`, `false`, or `null` for disputed.

- **`WordOrder::Free` and `VarietyKind::Historical`**, which the port dropped because
  both names were taken (`StressPattern.Free`, `ScriptStatus.Historical`). They are
  `WordOrder.FreeOrder` and `VarietyKind.HistoricalVariety`, and the wire keeps v1.x's
  `"Free"` and `"Historical"`. A v1.x document holding either used to be unreadable.
  No pre-built profile or variety uses them.

- **`registry_info_to_json` / `_from_json`.** New, not a restoration: v1.x's `LanguageInfo`
  did not derive `Serialize`. It keeps v1.x's `script_codes` name, and every field but
  the three Options is required. Coordinates stay integer milli-degrees, and a reader
  holds them to ±90,000 and ±180,000.

- **`-D DAIMON` and `-D HOOSH` encode their v1.x types**: `capability_*` and
  `agent_registration_*`; `language_query_*`, whose struct variants are externally tagged
  (`{"IdentifyLanguage":{"text":".."}}`), and `query_response_*`. `-D MCP` adds
  `tool_result_*`, for `{"Success":..}` / `{"Error":".."}`. `confidence` stays per-mille
  in the record and is v1.x's number on the wire, so 1000 is `1.0`. A value finer than a
  thousandth rounds on read, and one outside 0.0–1.0 is refused.
  `hoosh_answer_from_data` restores `structured_data`: the phoneme, as JSON text, where
  the port always returned none.

- **`-D MCP`: tool input schemas**, which the port dropped. v1.x's `ToolSchema` declared
  each tool's parameters and which were required, and the port's `ToolDef` kept only the
  string `"object"`, so an agent listing tools could not see what to pass.
  `tool_def_properties`, `tool_def_required`, `tool_param_name` and `tool_param_schema`
  read them, and there are two ways to write them, because v1.x had two:

  ```json
  {"name":"varna_script","description":"..","input_schema":{"type":"object","properties":{"code":{"description":"..","type":"string"}},"required":["code"]}}
  ```

  - `tool_def_to_json` and `tool_definitions_to_json`, with `_from_json`, are the serde
    form above, which is what v1.x's test round-tripped.
  - `tool_input_schema_to_json` writes the schema alone. A host answering `tools/list`
    wants this under its own `inputSchema` key, the name MCP uses and bote sent.

  A property is held as JSON text, since v1.x's was a free-form `Value`, so a decoded one
  keeps keywords varna does not write. bote's `version`, `deprecated` and `annotations`
  were always unset here, so they are not held. `varna_translate_ipa`'s description gets
  its `↔` back, and keeps the sentence 2.1.5 added. `mcp_tool_definitions` is now cached.

- **`dist/varna-json.cyr`** (`cyrius distlib json`): the engine plus `src/json.cyr`.
  It settles how bayan reaches consumers. `dist/varna.cyr` does not call bayan, so
  existing consumers carry no risk and no new dependency. A consumer that wants JSON
  folds this bundle instead and declares bayan its own way. Built and run as scratch
  consumers:

  | Bundle | bayan declared as | Result |
  |---|---|---|
  | `dist/varna.cyr` | nothing | builds, 380,216 B |
  | `dist/varna-json.cyr` | `dist/bayan-json.cyr` profile | builds, round-trips, 533,640 B |
  | `dist/varna-json.cyr` | `bayan` monolith, as shabda and shabdakosh do | builds, round-trips, 1,383,120 B |
  | `dist/varna-json.cyr` | nothing | refused at build, naming the `bayan_json_*` calls |

- **By-name codecs** through `#derive(Serialize)` on every enum on the wire: 8 in
  `src/phoneme.cyr` and 26 more across the modules. `E_to_json(v, sb)` writes `"Plosive"`,
  and `E_from_json_str(text)` returns `Ok(value)`, or `Err` for an unknown name. An enum
  varna added after v1.x has no v1.x name to keep, so its constants go on the wire as
  they stand: `"ROLE_ALLOGRAPH"`, `"ALIGN_NOMINATIVE_ACCUSATIVE"`, `"END_SAFE"`. Every
  spelling is now a contract, and `test_wire_names` writes each one out, since a rename
  moves the writer and the reader together and passes any round trip.

- **`tests/varna.fcyr` fuzzes the decoders.** It was a stub that decoded nothing. It now
  mutates seeds for all 26 decoders over 300,000 rounds, and checks two things: nothing
  crashes, and anything that decodes re-encodes to text that decodes to itself. 36,263 of
  the documents decoded. It passes under `--bounds` and allocator poison, and CI runs it.

- **`tests/json_roundtrip.tcyr` (369 assertions) and `tests/json_surfaces.tcyr` (155).**
  - All 26 serde tests from v1.x's `serde_roundtrip.rs`, and its hoosh and daimon ones.
  - Documents worked out by hand from the v1.x source and compared byte for byte, so
    they pin the format rather than this writer agreeing with this reader.
  - Every enum variant by name, with every spelling written out, and malformed cases for
    every field of every type.
  - Each MCP schema held against `mcp_invoke` in both directions: what it requires is
    enough to succeed, and leaving out any one gives that name's error. A schema that
    drifted from its handler would otherwise still parse.
  - Each hostile transform runs against all 19 core decoders: every truncation of every
    document, trailing garbage, a byte order mark, duplicate keys. Also 200,000 open
    brackets, a 100 KB name aimed at the 48-byte stack buffer, and RFC 3629's boundaries.
  - **Mutation-checked** on the final code: 155 deliberate defects after an unmutated
    control. 149 fail an assertion, 5 crash because what they remove is the guard against
    that crash, and 1 is equivalent, since every caller of `_json_u32` already refuses
    any negative. Earlier survivors found 4 real gaps, now covered (an escaped quote
    flipping what counts as inside a string, an explicit `null` kept as the text
    `"null"`, a renamed constant passing every round trip, and a tool's `type` lost on
    read), and 3 pieces of dead code, now removed.

- **Benchmarks** for every module's serializers: 20 rows, on `BENCHMARKS.md`.

### Fixed

- **A JSON reader could accept a name that was never written, or lose the end of a
  string.** JSON can spell U+0000 and a C string stops at it, so
  `"manner":"Plosive\u0000junk"` decoded as `Plosive`, and `"ipa":"p\u0000q"` decoded as
  `p`. Every reader now refuses both: the first is an unknown name, and the second is a
  value varna cannot hold.
- **A JSON reader accepted what RFC 8259 and serde refuse.** bayan's parser lets through a
  raw control byte inside a string and invalid UTF-8, and neither can be seen after
  decoding: an escaped `\n` and a raw newline are the same byte by then, and invalid
  UTF-8 kept in a record comes back out as a document a strict reader refuses. All
  readers now come through `_json_parse`, which checks the text first. That is about 8%
  of a read, and bayan's parser is the better home for it (roadmap).
- **`varna_compare` wrote into a fixed 4,096-byte buffer**, where running out truncated
  the document into invalid JSON. Every handler now builds in a `str_builder`, and all
  2,601 language pairs parse. `_mcp_append_json` and its helpers are gone.

### Changed

- **`src/phoneme.cyr` includes `lib/result.cyr`**, and the bundles' sidecar is now
  `result hashmap` (was `hashmap`). The enum decoders return a `Result`, but
  distlib's compile-verify did not record that leaf on its own: a decoder nobody
  calls leaves `Ok` / `Err` only as warnings. A consumer that called one would have
  failed to build. A scratch consumer declaring only `hashmap` now builds against the
  bundle with no warning and runs both codec directions. Include-once keys on the
  spelling `[deps] stdlib` auto-prepends, so varna's own builds read the file once.
- **`src/json.cyr` includes `lib/fnptr.cyr`**, and `dist/varna-json.deps` is
  `result fnptr hashmap`. Its readers call the derived decoders through one `_json_enum`,
  which replaced 15 identical wrappers. bayan is deliberately not in that sidecar, since
  which bayan to use is the consumer's choice.
- **Rule writers refuse a corrupt environment** (-1, nothing written). One out of range
  used to be written as `null`.
- **Binary size** (2.5.1 → 2.5.2): default 470,392 → 546,512 B, full `-D` 489,040 →
  569,840 B, `CYRIUS_DCE=1` 44,408 → 63,184 B.

### Performance

Interleaved against the commit before this work: **no non-JSON row separates** over four
runs, so `allophone_realize`'s value comparison does not show above noise, and JSON writes
are unchanged. **JSON reads are about 13% slower**, the cost of the stricter readers
(median of five runs): `json_inventory_read_en` 164.0 → 185.7 µs,
`json_phonotactics_read_en` 14.3 → 16.2 µs. Measured by taking it out, `_json_parse`'s scan
is about 8% of a read, two-thirds of that rise. The rest is the per-string NUL checks and
the indirect call in `_json_enum`.

The new rows (median avg):

| module | write | read | document |
|---|---|---|---|
| `script` | 2.1 µs | 8.5 µs | Devanagari |
| `numerals` | 17.4 µs | 74.6 µs | Greek isopsephy |
| `transliteration` | 18.0 µs | 50.9 µs | Devanagari → IAST |
| `grammar` | 3.0 µs | 15.4 µs | English |
| `lexicon` | 31.0 µs | 101.9 µs | Arabic Swadesh-25 |
| `cognate` | 4.9 µs | 13.5 µs | water |
| `dialect` | 2.3 µs | 9.0 µs | British English |
| `registry` | 2.4 µs | 8.1 µs | English |

Earlier in 2.5.2 no benchmarked function changed, but several rows moved, so these
microbenchmarks are **sensitive to code layout**. Interleaved against a build of the 2.5.1 tag on the same
toolchain, three runs each (avg):

| benchmark | 2.5.1 | this tree | |
|---|---|---|---|
| `grammar_by_code_lookup` | 244–246 ns | 213–214 ns | −13% |
| `lexicon_find_word` | 778–781 ns | 861–862 ns | +10.5% |
| `dialect_apply_overlay` | 2.959–2.972 µs | 3.116–3.150 µs | +5.7% |
| `swadesh_by_code_lookup` | 102–103 ns | 106–107 ns | +4.5% |
| `phonotactics_is_permitted` | 134–135 ns | 139–140 ns | +3.7% |

`grammar_by_code_lookup` is back near its 6.6.2 figure (209 ns). The slowdown 2.4.3–2.5.1
recorded across three toolchain pins was layout too, not codegen, which changes what
roadmap 2.5.4 should investigate.

The phonology bite moved rows again, in the recorded runs 17:42:22Z → 17:52:39Z:
`lexicon_find_word` −13% (845 → 735 ns, back at its 2.5.1 level), `dialect_apply_overlay`
−8.1%, `phonotactics_is_permitted` −8.0%, `registry_all_codes_iter` +9.5% (a row that
has swung ±10% between runs throughout). **These were not A/B'd interleaved this
time**, because the session was cut short. None of those functions changed, which is
consistent with the layout finding above.

## [2.5.1] - 2026-11-05

### Found — serialization was never ported (roadmap 2.5.2, P0)

The v1.x crate derived `Serialize` / `Deserialize` on 44 public types (corrected from "47" after release; 21 structs, 23 enums) across 13
modules, called `serde_json` in 11, and round-tripped every public type in
`tests/serde_roundtrip.rs` (27 tests). **The Cyrius port carried none of it.** No
varna type emits or reads JSON, and no varna code calls bayan. The only JSON in the
tree is the hand-built output in `src/mcp.cyr`. Meanwhile the README, the architecture
overview, the usage guide (which showed a `json_build(reg)` call that does not exist),
ADR 0001 and the `cyrius.cyml` dependency map all described it as done.

This came to light because the toolchain bump made bayan expensive (below), and
checking what varna uses bayan for turned up nothing. ADR 0002 records the port's
"all four" standalone test suites as ported, but its list swaps `mcp_json.tcyr` in for
`serde_roundtrip.rs`. Its "all 150 `pub fn`" parity check could not catch the gap,
because serde derives are not `pub fn`s. Restoring serialization is now the **P0 at
the top of the roadmap, targeted at 2.5.2**. The false claims are corrected in this
release, and ADRs 0001 and 0002 carry dated amendments.

### Changed

- **Toolchain `6.7.9` → `6.7.18`.** No data or API change, and no varna source change
  beyond the daimon version string. **2409 assertions** across 35 files pass, also
  under `cyrius test --bounds tests`. `fmt --check`, `lint`, `vet` (21 deps,
  0 untrusted), `deny` (0 violations) and `doc --check` are clean. 6.7.10–6.7.18 move
  a great deal of struct-value, `Str`, multi-value-return and declaration-zone
  semantics. varna's code was swept for every construct those changes reach
  (comments and strings stripped), and none occur: no `struct`, `Str`, tuple, `f64`,
  `ret2`, `: stack` pair, `**T`, closure, `impl` or narrow scalar annotation, no
  duplicate global or fn, and one spelling per include path.

- **bayan: the stdlib monolith → the thin `bayan-json` profile.** At 6.7.18 the
  stdlib's `lib/bayan.cyr` (bayan 1.11.1) includes `math` and the 7-file
  `lib/unicode/` family, about 353 KB of tables that DCE cannot drop. Every varna build
  prepends `[deps].stdlib`, so all of it landed in every binary. bayan ships one dist
  profile per format, and serde / serde_json map onto JSON only, so varna now takes
  `dist/bayan-json.cyr` as a named dep (`[deps.bayan]`, tag 1.11.1, the bayan the pin
  vendors and the same declaration mihi, samay, iam and ai-hwaccel use). `"bayan"` is
  gone from `[deps].stdlib`, since declaring both would define 27 JSON symbols twice.

  | build | 6.7.9, monolith | 6.7.18, monolith | 6.7.18, `bayan-json` |
  |---|---|---|---|
  | default | 746,824 B | 1,319,880 B | **470,392 B** |
  | full `-D` | 765,472 B | 1,338,528 B | **489,040 B** |
  | `CYRIUS_DCE=1` (live) | 70,984 B | 414,664 B | **44,408 B** |

  So 2.5.1 binaries are 37% smaller than 2.5.0's, not 77% larger. The dep is
  `optional = true` with `[features] default = ["bayan"]`, the ai-hwaccel pattern.
  Features resolve from the root manifest only, so varna's own builds and tests get
  it and consumers do not. That matters: shabda and shabdakosh declare the bayan
  monolith themselves and would otherwise vendor both files. Whether the dep stays
  optional once serializers enter the bundle is a 2.5.2 decision.

- **Lock: 34 → 35 entries.** `lib/bayan.cyr` is replaced by `lib/bayan-json.cyr`, and
  `lib/math.cyr` is added (the profile includes it). There is also a `commit` row
  pinning bayan 1.11.1 to `965298d`, checked against bayan's own tag, and the
  toolchain row is now `cyrius 6.7.18`. `cyrius deps --verify`: 35 verified, 0
  failed; `cyrius deps --locked` is clean. `cyrius deps` now fetches bayan from
  GitHub at the tag.

- **`dist/` sidecars: 4 leaves → 1 (`hashmap`).** distlib's compile-verify finds that
  `hashmap`'s own requires block brings `alloc`, `string`, `vec` and `str`. To check
  that before trusting it, a scratch consumer declaring only `hashmap` was built
  against the 2.5.1 bundle and run (`phoneme_has(phoneme_english(), "θ")` → 1).

- **bote recipe `3.4.10` → `3.8.14`**, bote's latest release (2026-11-05), which pins
  6.7.18. At 3.8.14 the core bundle is 15 modules (adding `regex` and `format`) and
  needs cyrius ≥ 6.7.13. Its sidecar is now `hashmap sys`: bote dropped `sigil` and
  `ct` once cyrius 6.7.13's distlib stopped recording a named dep's kept fold, so the
  `-D MCP` wiring no longer drags the crypto fold in. The block is still commented
  documentation and is not linked.

### Fixed

- **Docs that described serialization as done**: the README dependency stack, the
  overview's dependency map, "Serializable" claim and "structured Cyrius `struct`s"
  claim (varna's records are `load64` / `store64` layouts with no `struct`
  declarations), the usage guide's `json_build(reg)` example, and the `cyrius.cyml`
  map. Each now says what is true and points at roadmap 2.5.2.
- **The daimon registration version is `2.5.1`.** The `tests/daimon.tcyr` guard failed
  until it was edited, as on the last bump.

### Performance

The binary sizes above are the headline. `lib/bench.cyr` is byte-identical between
6.7.9 and 6.7.18, so the recorded rows compare directly: 2026-11-05T17:02:48Z on 6.7.9
(timer floor 1.371 µs) and 17:13:45Z on 6.7.18 (1.383 µs). The five largest movers were
then run interleaved, three runs per pin, against a 6.7.9 build of the 2.5.0 tag (avg):

| benchmark | 6.7.9 | 6.7.18 | |
|---|---|---|---|
| `phoneme_lookup_ipa` | 374–375 ns | 336–340 ns | **~10% faster**, every run |
| `grammar_by_code_lookup` | 227–231 ns | 241–244 ns | **~5.5% slower**, every run |
| `lexicon_find_word` | 784–794 ns | 767–776 ns | ~2% faster |
| `gematria_string_value_in` | 829–841 ns (one run 971) | 849–860 ns | noise |
| `registry_all_codes_iter` | 3.85–4.66 µs | 4.01–4.72 µs | noise, both ways |

The two consistent effects can't be attributed to one cause. The 6.7.18 build differs
in both the compiler and the bayan file prepended to every compile, which changes code
layout. `grammar_by_code_lookup` (`"ru"`, nine `streq` misses) has now slowed at three
consecutive pins: 209 ns avg on 6.6.2, ~229 on 6.6.20 and 6.7.9, and ~241 on 6.7.18.

## [2.5.0] - 2026-11-05

### Changed

- **Toolchain `6.6.20` → `6.7.9`, the Cyrius language minor.** No data or API change,
  and no varna source change beyond the daimon version string below. **2409
  assertions** across 35 files pass. `fmt --check`, `lint`, `vet` (21 deps,
  0 untrusted), `deny` (0 violations) and `doc --check` are clean, and both build
  profiles compile with no warnings.

  6.7.x changes what some existing code *means*, not just what compiles, so the tree was
  swept for each such construct (with comments and string literals stripped first).
  None occur in `src/` or `tests/`:

  | since | change | in varna |
  |---|---|---|
  | 6.7.3 | a lone `!` was silently dropped by the lexer (`if (!ok)` meant `if (ok)`); it is now a real operator | no `!` outside `!=` |
  | 6.7.9 | unary `-` / `~` on a constant binds tighter than a following literal operator (`-K + 10` was 15, now 5) | no unary op on a name |
  | 6.7.8 | `*p` through a narrow `*T` reads and writes the element width, not a word; signed slice elements read negative | no `*T` derefs, no slices |
  | 6.7.8–6.7.9 | a `for` step must assign a declared variable; stray tokens after it are refused | no `for` loops (all `while`) |
  | 6.7.0–6.7.8 | `trait`, `const`, `true`, `false`, `do`, `dyn` reserved; untyped `self` in an `impl` is `*T` | none used, no `impl` |

  As an extra check the suite was also run under 6.7.8's opt-in bounds-checked mode
  (`cyrius test --bounds tests`): 2409 / 2409 pass. That mode checks typed arrays and
  slices only, not varna's raw `load*` / `store*` accesses.

- **`cyrius tests` → `cyrius test tests`** in `scripts/check.sh`, the `Makefile`'s
  `test` target and the docs. 6.7.6 deprecated the old spelling, and 6.7.9 warns that
  the next release removes it.

- **Lock regenerated** (`rm -rf lib && cyrius deps`): the same 34 files at 6.7.9
  hashes, with the toolchain row now `cyrius 6.7.9`. `cyrius deps --verify`: 34
  verified, 0 failed. The new `cyrius deps --locked` (6.7.6) reports lib/ and the
  lock are exactly what the tags resolve.

- **`dist/` regenerated.** Only the `# Version:` header line changed, and both
  sidecars are still the same four leaves. The bundle body is byte-identical to
  2.4.3's, which was built and verified on 6.6.20.

- **bote recipe `3.3.17` → `3.4.10`**, the last bote tag pinned to 6.7.9. This is the
  commented `[deps.bote]` block, which documents the `-D MCP` wiring and is not linked.
  At 3.4.x the core bundle is 13 modules, adding `log`, and states a minimum of cyrius
  6.7.2. Its `log` module calls sakshi, which a `[deps.bote]` pin gets through bote's
  own libro dep. The sidecar is unchanged (`hashmap bayan sigil ct`). The recipe also
  notes that since 6.7.6 a `path` beside `git` / `tag` is a local-mode-only override.

- **Binary size** is flat. The default build went from 746,840 B to 746,824 B, the full
  `-D` build from 761,392 B to 765,472 B, and the `CYRIUS_DCE=1` build from 71,000 B to
  70,984 B.

### Fixed

- **The daimon registration version is `2.5.0`.** The `tests/daimon.tcyr` guard
  added in 2.4.3 failed on this bump before the string was edited, which is what it
  is for.

### Performance

Neutral. `lib/bench.cyr` is byte-identical between 6.6.20 and 6.7.9, so unlike 2.4.3
the recorded rows compare directly: 2026-11-05T16:52:29Z on 6.6.20 (timer floor
1.353 µs) and 16:56:15Z on 6.7.9 (1.378 µs). The four largest movers in those rows
(`gematria_string_value_in` +7.3%, `dialect_apply_overlay` +4.9%,
`gematria_char_value_ordinal` +4.9% on `min`, and `grammar_by_code_lookup`) were then
run interleaved, three runs per pin:

| benchmark | 6.6.20 min | 6.7.9 min |
|---|---|---|
| `gematria_string_value_in` | 813–820 ns | 817–835 ns |
| `dialect_apply_overlay` | 2.868–2.886 µs | 2.877–2.923 µs |
| `gematria_char_value_ordinal` | 496–501 ns | 495–498 ns |
| `grammar_by_code_lookup` | 228–229 ns | 228–232 ns |

All four are within ~1–2%, so the recorded deltas were run-to-run drift.
`grammar_by_code_lookup`'s slowdown since 6.6.2 (see 2.4.3) is not recovered on 6.7.9.

## [2.4.3] - 2026-11-05

### Changed

- **Toolchain `6.6.2` → `6.6.20`.** No data or API change, and no compiler rejection:
  the only source edit is the daimon version string below. **2409 assertions** across
  35 files pass (2407 carried over + 2 new). `fmt --check`, `lint`, `vet` (21 deps,
  0 untrusted), `deny` (0 violations) and `doc --check` are clean, and both build
  profiles compile.

- **Lock: 31 → 34 entries, regenerated from scratch** (`rm -rf lib && cyrius deps`).
  The three new rows are leaves the 6.6.x stdlib now includes itself: `chrono`
  (included by `hashseed`), plus `poison` and `alloc_cx` (both included by `alloc`).
  The lock also gains a toolchain row (`cyrius 6.6.20`). From 6.6.20 a fresh
  `cyrius deps` writes the lock itself, so the separate `--lock` step is no longer
  needed. `cyrius deps --verify`: 34 verified, 0 failed.

- **`dist/` regenerated under 6.6.18+ `distlib`.** Each bundle now opens with a
  compile-verified `# Requires` block (`include "lib/string.cyr"`, `vec`, `alloc`,
  `hashmap`), so `include "dist/varna.cyr"` compiles on its own. The sidecars shrink
  because distlib's compile-verify is now the only authority over them. Before 6.6.18
  the whole `[deps].stdlib` was unioned in.

  | sidecar | 2.4.2 | 2.4.3 | dropped |
  |---|---|---|---|
  | `dist/varna.deps` | 16 | 4 | `fmt str slice syscalls io args assert bayan fnptr tagged result bench` |
  | `dist/varna-core.deps` | 6 | 4 | `assert str` |

  **Consumer impact:** a consumer that calls a stdlib family itself but got that leaf
  only through varna's old sidecar now has to declare it in its own `[deps] stdlib`.
  The 6.6.18+ CLI prints a `hint:` line naming the missing leaf. The current folders,
  shabda, shabdakosh and sankhya (all at varna 2.4.1), already declare broad stdlib
  lists of their own, so none of them should notice.

- **bote recipe `3.3.7` → `3.3.17`.** This is the commented `[deps.bote]` block in
  `cyrius.cyml`, which documents the `-D MCP` wiring and is not linked. 3.3.17 is the
  bote tag that pins 6.6.20. The 2.4.2 entry below says bote moved to 3.3.8, but the
  recipe's `tag` line was never edited and still read 3.3.7. At 3.3.17,
  `dist/bote-core.deps` is `hashmap bayan sigil ct`: chrono and the rest arrive
  through those leaves' own requires blocks, so wiring it no longer needs `chrono`
  added by hand. It does pull the sigil crypto fold into the `-D MCP` build. Per
  bote's own 3.3.17 notes no core module uses sigil or ct, and the leak is filed
  upstream as a cyrius distlib bug.

- **Binary size.** The default build grew from 683,376 B to 746,840 B and the full
  `-D` build from 702,032 B to 761,392 B. Almost all of that is dead stdlib code the
  6.6.x leaves now include themselves: unreachable bytes went from 618,665 to 675,744.
  Under `CYRIUS_DCE=1` the live binary grows from 64,880 B to 71,000 B.

### Fixed

- **The daimon registration reported version `2.4.1`.** The string is hardcoded in
  `src/daimon.cyr` and was missed at 2.4.2. It now reads `2.4.3`, and
  `tests/daimon.tcyr` reads `VERSION` and asserts the two match, so the next bump
  that misses it fails `cyrius tests`. The new test failed against the stale string
  before the fix, so it does catch the drift.

### Performance

Neutral, apart from one lookup that the toolchain made slightly slower. The 6.6.20
`lib/bench.cyr` is a different instrument: 6.6.9's "regime 6" stopped reporting a
minimum computed after subtracting the *mean* clock floor, which had made `min` read
low. So the raw recorded rows (`bench-history.csv`, 2026-11-05T16:37:07Z on 6.6.2 and
16:39:13Z on 6.6.20, timer floor ~1.35 µs) are not directly comparable. 17 of 22 rows
read faster and 4 slower, the slowest by +6.4% on `min`.

To separate harness from compiler, the 6.6.2 harness was built against 6.6.20
(`tests/varna.bcyr` including the old `lib/bench.cyr` under another path, with
`bench` dropped from the scratch `[deps].stdlib`, because `cyrius bench` re-vendors
`lib/` and auto-prepends the harness). Runs were interleaved, 2–3 each:

| benchmark | 6.6.2 (old harness) | 6.6.20 (old harness) | 6.6.20 (new harness) |
|---|---|---|---|
| `phoneme_lookup_ipa` min | 313–327 ns | 310–312 ns | 353–369 ns |
| `registry_all_codes_iter` min | 3.69–3.76 µs | 3.55–3.77 µs | 4.06–4.19 µs |
| `grammar_by_code_lookup` avg | 209–210 ns | 238–239 ns | 228–229 ns |

The first two only *look* slower: with the harness held constant they are unchanged,
and the gap is the new instrument. `grammar_by_code_lookup` (`"ru"`, nine `streq`
misses before the match) really is about 29 ns (~14%) slower on 6.6.20, with the
harness held constant. `streq` is byte-identical across the two pins, and
`script_by_code_lookup`, also a `streq` dispatch, got faster (109 → 104 ns avg), so
this is codegen or layout specific to that function, not the string helpers. It is
not chased here.

## [2.4.2] - 2026-09-11

### Changed

- **Toolchain `6.5.36` → `6.6.2`**, and bote `3.3.7` → `3.3.8`. No source change.
  Zero compiler rejections, zero fail-open sites; six `callptr` sites, none
  reaching a pair-returning target. **2407 assertions** pass unchanged.

  The bote bump matters: bote <= 3.3.7 pinned majra 2.7.0, whose `_sub_new/2`
  collides with libro's `_sub_new/1` — a hard error since cyrius 6.5.37 — and
  `dist/bote.cyr` at 3.3.7 still called the retired `payload(`.


## [2.4.1] - 2026-08-28

Three of the four open data-accuracy items from the roadmap. Every value was checked
by independent research and then adversarially refuted; **most proposed corrections
did not survive that, and were not made**. A reference corpus is damaged as much by
churn as by error, so a value defensible under any mainstream description stays — and
the variety it commits to is now named in the source comment.

### Fixed

- **Thai had no falling tone at all.** The row was `˧ ˨˩ ˨˩˦ ˦˥ ˩˧˥` — two rising
  shapes and no โท. It is now `˧ ˨˩ ˦˩ ˦˥ ˨˩˦` (33 / 21 / 41 / 45 / 214), all from
  Tingsabadh & Abramson (1993), the IPA Handbook illustration, in the canonical order
  สามัญ / เอก / โท / ตรี / จัตวา. `˩˧˥` appears in no description of Thai in any
  tradition.

  **The roadmap's own diagnosis of this was wrong, and dangerously so.** It read that
  `˨˩˦` "is the shape of Mandarin's third tone" and that its appearing in two
  unrelated languages "points at a copy-paste". But [214] is independently correct for
  Thai's rising tone *and* for Mandarin's third — both languages genuinely have a
  concave low rise. Acting on that text literally would have deleted Thai's one
  correct contour value. The roadmap entry has been rewritten and the reasoning kept
  as a record.

  Falling is `˦˩` [41], not the `˥˩` [51] the roadmap named — that is Gandour's value,
  and the other four slots are Tingsabadh & Abramson's. Mixing traditions inside one
  row is what produced the defect in the first place.

- **Vietnamese huyền was `˨˩˦`.** Here the copy-paste diagnosis *does* hold — that is
  Mandarin's third tone, and no description of any Vietnamese dialect gives huyền a
  dipping contour. It is a low fall, `˨˩` (Nguyễn Văn Lợi & Edmondson 1998, Northern),
  which the rest of the row already follows.

- **Burmese's checked tone was `˩ʔ`, a low level — the opposite of every description.**
  Wheatley (1987:842) has "the pitch of the checked tone (high, in citation)"; Chang
  (2008) and Gruber (2011) agree. `˩ʔ` was the corpus's generic checked placeholder,
  also used for Literary Chinese. Now `˥˧ʔ` — `˥˧` rather than Watkins' `˥˩` because
  `tone.cyr` derives register from the rounded mean of the levels, and `˥˩` would come
  out `TONE_MID`, the very register the fix exists to correct.

- **Wolof's vowel system was four qualities.** `/i e a o/` + long matches no analysis
  of Wolof — no description lacks `/u/`, and the block was byte-identical to the
  Classical Nahuatl one, i.e. a copy-paste rather than a reduced transcription. Now
  the full 8 short + 7 long of Ka (1994) / Unseth (2009): `/i u e o ɛ ɔ ə a/`, with
  `/ɛː ɔː uː/` added and no `/əː/` (Ka: "the vowel [ə] has no long form"). The CLAD
  orthography mapping and the ATR pairing are recorded in the source comment.

### Added

- **`phoneme_is_breathy(p)`** — true for `Breathy`, or for any **voiced** segment
  marked `Aspirated`. This closes the Sanskrit voiced-aspirate item **by deciding not
  to change the data**, which is the opposite of what the roadmap called for.

  Sanskrit spells its voiced aspirates with U+02B0 where Hindi, Bengali and Urdu use
  U+02B1, and the item read "decide one transcription and apply it". The review found
  the split legitimate: Sanskrit's inventory is a Pāṇinian varga listing — its comments
  are *Kavarga, Chavarga, Tavarga, Pavarga* — under which all ten mahāprāṇa form one
  natural class, the class Grassmann's Law and Bartholomae's Law operate over. ⟨bʰ⟩ on
  a voiced base is the IPA chart's own example for the diacritic. Retranscribing would
  have broken `phoneme_find(sa, "bʰ")` for every consumer.

  The premise behind the item was also wrong: the two spellings do **not** yield
  different distinctive-feature vectors. `features.cyr` derives `DF_SPREAD_GLOTTIS`
  from `Aspirated` and `Breathy` alike, so Sanskrit /ɡʰ/ and Hindi /ɡʱ/ were already
  bit-identical there. The real inconsistency was one level up, at the raw
  `PhonemeFeature` bit, and asking rather than spelling fixes it: "which segments are
  breathy?" now answers 5 for each of the four languages while each keeps its own
  transcription. The voicing test is load-bearing and pinned — /kʰ/ is not breathy in
  Sanskrit, Hindi or Zulu.

- **Thai and Vietnamese tone assertions.** Neither language had a single tone
  assertion anywhere in the suite, which is exactly why both errors survived to be
  found by reading. All five Thai tones and four Vietnamese tones are now pinned by
  index, which also pins the ordering convention the rows depend on. One assertion
  deliberately records that Thai จัตวา and Mandarin tone 3 share `˨˩˦` **correctly**,
  so the refuted copy-paste inference cannot be re-derived from the data.

### Changed

- Wolof's declared builder capacity 33 → 39 (24 consonants + 15 vowels). The old 33
  was already wrong against the actual 32.

### Verified and deliberately unchanged

Each of these was proposed as a correction and refuted on the sources:

- **Lao's six tones**, including its own `˩˧˥` — unlike Thai's, that is its genuine
  low-onset rise (Enfield 2007, *khaa3* 'leg'), and the proposed replacement would
  have deleted a tone.
- **Mandarin's neutral `˧`**, **Literary Chinese's four**, **Vietnamese nặng `˨˩ˀ`**,
  **Burmese low `˨˩`**, **Hausa and Somali as two-level systems** (both WALS "simple
  tone system"; Somali's tone-bearing unit is the mora, so the *beer* triplet needs no
  contour toneme).
- **Somali's five vowel qualities** — Orwin (1994), a named analysis, not a truncation.
  This is the one genuine remaining gap in the ATR item, and it is a choice of analysis
  rather than an omission.
- **The `AdvancedTongueRoot` bit still does not land.** Yoruba and Wolof are now
  markable, but Somali is not, and marking two of three would leave ATR silently absent
  from a language that has it. Somali first, then the bit.

## [2.4.0] - 2026-08-28

Gematria and numeric letter values: a unified character-to-number layer across five
scripts, and the fix for two defects it uncovered in the Greek table that has been
shipping since the port. Additive, plus a large speedup on numeral lookup.

### Added

- **numerals** — `NumericSystem` (`NUM_STANDARD` / `NUM_ORDINAL` / `NUM_REDUCED`)
  and a unified lookup across scripts: `char_value(script, system, ch)`,
  `string_value_in(script, system, s)`, `script_alphabet_values(script, system)`,
  `gematria_by_script(code)` and `gematria_all_scripts()`. 136 mapped characters
  over Hebrew, Arabic, Latin, Church Slavonic and Greek.

- **Four alphabet tables** — Hebrew mispar hechrachi (22 letters + 5 finals),
  Arabic abjad in **Mashriqi** reckoning (28, abjadi order, not hija'i), Latin
  simple gematria (a=1..z=26), and Church Slavonic (27, standardised recension).
  Greek **reuses** `numerals_greek_isopsephy` rather than copying it — a test
  asserts the identity, so the two can never drift.

- Only `NUM_STANDARD` is stored. Ordinal is derived from alphabet position and
  reduced is the standard value's digital root, so neither can drift from the
  values they describe. The roadmap also listed an "Additive" `NumericSystem`
  value; it was **not added**, because it describes how a system combines values
  across a string rather than a per-character method, and that axis already
  exists as `NumeralSystemKind`.

- **`LetterRole`** on every `NumeralMapping` — `ROLE_LETTER`, `ROLE_ALLOGRAPH`
  (a positional variant such as Greek ς or the Hebrew finals) and
  `ROLE_NUMERAL_SIGN` (a value-bearing sign that is not a letter). Accessors
  `numerals_mapping_char` / `_len` / `_value` / `_role` / `_base`.

- **tests/gematria.tcyr** — 265 assertions. Every one of the 136 values is pinned
  individually against the canonical sources, because a structural sweep cannot
  see two letters swapping values: the multiset is unchanged and every rung still
  resolves. Plus the derivation rules, the absences, the Greek reuse identity,
  the accessors at each role, and a corpus sweep that pins each alphabet's size
  as its **maximum** position rather than its last letter's.

- **An allocation assertion**, not just a benchmark — `string_value_in` and
  `char_value` are measured with `alloc_used()` deltas over 1,000 calls and must
  come back at exactly 0 bytes on the success path, the unmapped-character path,
  and the rescanning ordinal path. Assertions cannot see a leak and this release
  shipped one during development (below); a measurement is the only thing that
  catches it.

- **Four benchmark rows** for the new surface — `gematria_char_value`,
  `gematria_char_value_ordinal`, `gematria_string_value_in` and
  `gematria_alphabet_table`. None of 2.4.0's public API had a row, so the
  benchmark gate could not have seen a regression in it.

### Fixed

- **Greek ordinals were shifted from tau to omega.** Final sigma sat inline in
  the isopsephy table, and the ordinal was derived from a mapping's raw vec
  index — so ς claimed a position of its own and every letter after it read one
  too high (τ=20, ω=25). ς is a positional allograph of σ, not a 25th letter.
  Now τ=19 and ω=24, and ς folds to σ's position.

- **Greek could not compute 666.** The letter run skips 6, 90 and 900 because
  those values belong to three numeral-only signs — stigma ϛ, koppa ϟ, sampi ϡ —
  which were absent entirely, so `χξϛ` returned the unmapped sentinel. All three
  are now mapped as `ROLE_NUMERAL_SIGN`, giving a complete 1..900 run.

- **The root cause of both**, rather than the two symptoms: deriving an ordinal
  from a raw table index assumes every entry occupies an alphabet position.
  `_ordinal_at` now counts only `ROLE_LETTER` entries. A consequence is that the
  Hebrew finals fold to their base letter's position (final mem is 13, like mem),
  which is what classical mispar siduri does; they previously read 23-27.

- **The architecture overview's numeral table listed systems that do not exist**,
  predating this release: "Arabic-Indic Digits" and "Egyptian Fractions" were never
  in the code (`Arab` is the abjad; `Egyp` is Hieroglyphic Numerals), and Chinese
  rod numerals were missing entirely. Corrected against the constructors, and the
  usage guide's new gematria examples are executed as assertions rather than
  written by hand.

- **A per-character allocation leak in `string_value_in`**, introduced earlier in
  this release and caught before it shipped. It cut a NUL-terminated copy of every
  codepoint purely to call `char_value`, burning 8 B per character per call — on
  the failure path too, since the copies were made before the unmapped character
  was reached. The bump allocator never frees, so that is permanent, unbounded in
  string length, and the exact pattern `_numerals_value_of_bytes` already existed
  to remove. `_char_value_bytes` now compares in place, and `string_value_in`
  resolves the script once instead of once per character.

- **`_ordinal_at` would have dereferenced a null base.** No constructor produces
  an allograph without one, but 0 is the not-found sentinel in that slot and
  `strlen` would have followed it. Guarded.

- **`script_alphabet_values` allocated before checking its sentinel**, leaking a
  vec on every not-found call.

- **Church Slavonic rationale was wrong in a way that would propagate.** The
  comment claimed only letters with a Greek ancestor carry a value. Its own table
  refutes that: ц descends from Hebrew tsade and carries 900, and ч is Slavic and
  carries 90. The rule is positional — values follow the Greek numeral *sequence*
  slots, including those held by stigma, koppa and sampi. The wrong rule had been
  mirrored into the tests, so it is corrected in both places and both
  counterexamples are now asserted. No value changed.

### Changed

- `NumeralMapping` grew 16 B → 40 B: `[0]=char [8]=len [16]=value [24]=role
  [32]=base`. All five fields have accessors; nothing outside `numerals.cyr`
  decodes the record by offset any more.

- The Arabic and Church Slavonic records now **name their recension**
  ("Arabic Abjad (Mashriqi)", "Church Slavonic Numerals (standardised
  recension)"). "Abjadi" alone is ambiguous: the Maghrebi tradition moves exactly
  six letters, and scoring a Maghrebi source against a Mashriqi table is silently
  wrong on those six and only those. Documented, not encoded — variant glyphs and
  period alternatives (izhitsa, ot, koppa-for-90, Glagolitic) stay out of the
  canonical tables, because the mappings vec is also the ordinal source and an
  inline variant corrupts every position after it. That is not hypothetical: it
  is exactly what final sigma did to Greek.

### Performance

- **Numeral lookup is ~40% faster.** The scan called `strlen()` on each stored
  character once per entry per lookup — recomputing a constant. The byte length
  is now cached at construction, so the scan compares an integer before touching
  memory. Measured over 4 interleaved rounds, non-overlapping spreads:
  - `numeral_value_of_char` 279ns → 168ns (**-39.8%**), spreads 279-295 / 168-179
  - `numeral_string_value_word` 892ns → 555ns (**-37.8%**), spreads 892-939 / 555-588

  This is a net win despite the Greek table growing by three entries, which adds
  a comparison to every lookup that scans past stigma or koppa.

- **The string walk is 25.8% faster** than the leaking version it replaced, since
  removing the per-character copy also collapsed a per-character script lookup to
  once per string. Interleaved, 3 rounds, non-overlapping spreads:
  `gematria_string_value_in` 1012ns → 751ns, spreads 1012-1049 / 751-764.
  `gematria_char_value` is unchanged at 262-266ns, as expected — its path did not
  move.

- **The ordinal table build is no longer quadratic.** `script_alphabet_values`
  called `_ordinal_at`, itself a full rescan, once per entry. Letters now take a
  running counter and only allographs and numeral signs — at most a handful per
  table — still rescan.

## [2.3.2] - 2026-08-28

Final slice of the 2.3.x Typological Depth line: vitality and geography on every
registry entry. Also bumps the Cyrius pin to 6.5.36. Additive.

### Added

- **registry** — `EndangermentLevel`, `MacroArea`, and latitude/longitude on all 51
  languages, with `registry_info_endangerment` / `_macroarea` /
  `_latitude_mdeg` / `_longitude_mdeg`, plus `registry_by_macroarea(area)` and
  `registry_at_least_endangered(level)`.

- **`END_HISTORICAL`**, an eighth vitality value: no native-speaker community, but
  the language remains in active scholarly, liturgical or literary use. Latin,
  Ancient Greek, Literary Chinese, Sanskrit and Classical Nahuatl take it.
  Collapsing those into `END_EXTINCT` would conflate them with a minority language
  whose last speakers died. **No corpus language is `END_EXTINCT`**, which a test
  pins so the distinction stays visible. A threshold query deliberately excludes
  the historical languages rather than sweeping them in, since `END_HISTORICAL`
  sorts last numerically but is not "worse than extinct".

- **tests/vitality.tcyr** (65 assertions), including three checks a per-row review
  cannot do: every coordinate is validated against its own macro-area's bounding
  box, every coordinate must be a multiple of 100 mdeg so the storage cannot
  outrun the one-decimal source, and `ko` is pinned present by name.

### Fixed

Four vitality values, from an audit aimed at the low-confidence and
indigenous-language rows:

- **`haw`** Moribund → **Threatened**. Moribund means only the grandparent
  generation speaks it, which is false: Ni'ihau transmission has never broken and
  the Pūnana Leo / Kula Kaiapuni immersion system has run since 1983. The old value
  traces to UNESCO's Atlas, **frozen in 2010, before the revitalization**.
- **`qu`** Shifting → **Vulnerable**. Downstream of varna's own 2.3.1 rename: the
  entry was scoped to *Southern Quechua* (~5–7M speakers, intact rural
  transmission, co-official) while the level still described the whole
  macrolanguage. Shifting is right for Central and northern Quechua, which the
  entry excludes by name.
- **`nah`** Shifting → **`END_HISTORICAL`**. Same cause. The 2.3.1 rename to
  *Classical Nahuatl* was correct — the inventory has no voiced stops, the
  four-vowel system with contrastive length, /t͡ɬ/, /kʷ/ and the saltillo — so it
  was the level that described a different language.
- **`ko`** was missing entirely from the gathered data (50 of 51 rows) and is
  sourced here: Safe, Eurasia, ~80M speakers, official in both Koreas.

- **docs/development/typology-data.md** synced to the code; it had drifted on all
  four historical languages and omitted `ko`.

### Changed

- **Toolchain** — Cyrius pin `6.5.35` → **`6.5.36`**. The 6.5.35 stdlib snapshot had
  been rewritten in place locally (`io.cyr` plus four `syscalls_*` files in varna's
  closure), which `cyrius deps --verify` caught in the gate — the check added at
  2.1.1 for exactly this. The lock now matches a version that actually changed:
  29 verified, 0 failed. The five files in the re-lock are precisely those five.
- **LanguageInfo** is 80 bytes, up from 48.
- Every row whose ISO code is broader than the entry (`ar`, `zh`, `gn`, `nah`, `qu`)
  now carries an explicit SCOPE comment naming the variety its values describe.

### Notes

- **Correction to the 2.3.2 working notes: Cyrius does have f64.** An earlier
  comment in this work claimed it did not, and justified the integer coordinate
  encoding on that basis. That was wrong — `lib/math.cyr` and `lib/ganita.cyr`
  carry a full float surface (`f64_pow`, `f64_hypot`, trig, `f64v2`/`f64v4` SIMD)
  and bayan parses JSON floats. The claim came from misreading a design note in
  `hoosh.cyr` ("per-mille … to avoid f64") as a statement about the language.

  Milli-degrees are **kept**, on the real reasons: the source is rounded to one
  decimal degree so a float would advertise precision the data lacks, and integers
  compare exactly so the bounding-box and precision tests need no epsilon. The
  comments now say that instead.

- **The failure pattern behind three of the four fixes** is one thing: a level
  assigned to a different language than the entry names. A macrolanguage or
  collection ISO code paired with a single vitality value is structurally risky,
  which is why the SCOPE comments and their tests exist.

### Performance

- **Flat.** `registry_all_codes_iter` -0.3%, `registry_phonemes_lookup` 0.0%. Two
  rows tripped a 10% threshold and both are min-of-N artifacts at the harness floor
  (`english_phoneme_inventory` 5→3 ns on a new spread of 3-5; `greek` 5→6 on 5-6).


## [2.3.1] - 2026-08-28

Second slice of the 2.3.x Typological Depth line: genealogical classification on
every registry entry. Additive — the three original `LanguageInfo` fields and
their accessors are untouched.

### Added

- **registry** — `family`, `subfamily` and `genus` on all 51 languages, with
  `registry_info_family` / `_subfamily` / `_genus`, the `registry_family(code)`
  shortcut, and `registry_by_family(family)` returning every code in a family.
  An entry built without a classification reads back `0`, which is
  distinguishable from any placeholder string.

- **registry** — a written **rank rule** in the module header, which is what the
  file lacked and what let the defects below through: family is the conventional
  top-level stock, subfamily is always a *primary* (depth-1) branch, genus is the
  smallest conventionally named clade the language's close relatives still share.
  Two consequences are documented as correct rather than left to look like
  oversights: genus depth varies between families because family diversity does,
  and `genus == subfamily` is the right answer for a low-diversity family (seven
  rows do this). `la` is recorded as the one intentional exception to non-nesting.

- **tests/classification.tcyr** (176 assertions), including two groups that exist
  because of the audit: `non_nesting`, which checks the invariant a rank column
  must satisfy — no value may be a proper ancestor of another in the same column —
  and `subfamily_is_always_a_primary_branch`. Plus `rejected_groupings`, scanning
  all three fields of all 51 rows for **Altaic, Ural-Altaic, Nostratic,
  Hamito-Semitic, Mon-Khmer and Quechumaran**.

### Fixed

Three **partition violations**, found by a systematic audit after the row-by-row
review had passed them, and fixed before release:

- **`ar`** genus was `Central Semitic`, which strictly contains `he`'s
  `Northwest Semitic` — so grouping by genus asserted Hebrew sits outside Central
  Semitic. Now `Arabic`, the actual sister node.
- **`sa`** genus was `Old Indo-Aryan`, a stratum *inside* the `Indo-Aryan` that
  `hi`/`ur`/`bn` carry, not a sister of it. Now `Indo-Aryan`, with the stage in a
  comment — the same treatment `grc` and `lzh` already had. This value also
  contradicted the table it was derived from, which had the stage as a
  parenthetical.
- **`haw`** subfamily was `Oceanic`, four nodes inside the `Malayo-Polynesian`
  that `id` and `tl` carry. It was the only row in all 51 whose subfamily was not
  a primary branch.

- **tests/classification.tcyr** had *locked one of these in*: it asserted
  "Sanskrit and Hindi are different genera", which is only true while the nesting
  bug exists. Inverted to `_same_genus("sa", "hi")`.

### Changed

- **LanguageInfo** is 48 bytes, up from 24.
- **`qu`** is named *Southern Quechua* and **`nah`** *Classical Nahuatl*, in both
  the registry and the inventory. Both ISO codes have wider scope than the entry
  describes — `qu` is a macrolanguage spanning both Quechua branches and `nah` a
  639-3 collection code — so the previous names made the subfamily false for part
  of that scope. Naming the entry for the variety it actually models is the
  pattern `zh` already used. The codes are untouched.
- Comment pass on the four Niger-Congo rows, which asserted `Niger-Congo` in the
  field while using `Atlantic-Congo` in their own prose, and on `am`, whose
  "South Semitic" node is superseded. Fields unchanged — only the marked set is
  now complete rather than arbitrary.

### Notes

- **Caveats live in comments, not in fields.** The source table carried values like
  `Ugric (contested; Glottolog rejects the node)` — prose inside a queryable field.
  The encoded value is the clean conventional label with the objection beside it.
  Four subfamilies are contested this way (`hu`, `wo`, `my`, `qu`), each pinned by a
  test so the choice stays visible in code.

- **The audit is why this release is correct.** Four independent lenses over the
  whole table at once — granularity, cross-row coherence, rejected nodes,
  macrolanguage treatment — after a row-by-row review had returned **zero
  disputes**. Every one of the three bad values was individually defensible, which
  is exactly why per-row review could not see them: the defect was only visible in
  the column read as a whole. The reintroduced-defect check confirms the new
  invariant catches all three (9 assertions fail, including 5 genus nestings and 2
  subfamily nestings from the generic scan).

- **Re-levelling the genus column was considered and rejected.** Two auditors
  flagged the same depth variance and prescribed opposite fixes — one to flatten to
  the WALS tier, one to deepen. When careful reviewers reach opposite conclusions
  from the same column, there is no single right depth to reach for; and the churn
  would touch all 51 rows, the assertions, both bundles and the doc table for no
  consumer-visible gain, since no `registry_by_genus` exists yet. The rank rule is
  written down instead, to be revisited if that accessor lands.

### Performance

- **Flat.** The rows this release could touch are unmoved: `registry_all_codes_iter`
  -0.4%, `registry_phonemes_lookup` 0.0%. Three other rows tripped a 10% threshold
  and all three are min-of-N artifacts — `allophone_realize` showed "-78.8%" on a new
  spread of 7-33 ns, i.e. one outlier sample against a stable 33. Reporting spreads
  alongside minima is now routine here precisely because this statistic has produced
  a phantom result in three consecutive releases.

## [2.3.0] - 2026-08-27

First slice of the 2.3.x Typological Depth line: eight WALS-style dimensions on
`GrammarProfile`. Additive — the eight original fields and their accessors are
untouched.

### Added

- **grammar** — eight dimensions across the 11 language profiles, with accessors
  `grammar_alignment`, `grammar_adposition_order`, `grammar_tense_marking`,
  `grammar_has_future`, `grammar_evidentiality`, `grammar_negation`,
  `grammar_article_type`, `grammar_adjective_order`, `grammar_relative_order`,
  and five new enums.

  Every dimension carries an explicit "no dominant value" variant, and three
  profiles use one: Mandarin's adposition order (WALS 85A), Arabic's
  tense-vs-aspect question, and Korean negation. `grammar_has_future` is a
  tri-state (1 / 0 / -1) because Korean's `-gess-` is genuinely disputed.

- **tests/typology.tcyr** (73 assertions) pinning the well-established cases —
  Hindi split-ergative, French discontinuous negation, Mandarin and Japanese
  prenominal relatives, the five article-less profiles — so a regression reads as a
  linguistic error rather than a diff. Plus range checks and a guard that all
  eleven profiles moved off the constructor defaults.

- **docs/development/typology-data.md** — the verified classification and
  endangerment/geography tables for **2.3.1** and **2.3.2**, so that verification
  is not repeated. Carries its own caveats.

### Changed

- **GrammarProfile** is 136 bytes, up from 64. Only 11 instances exist.
- **roadmap** — the "Missing script entries" sub-item is marked shipped: all nine
  scripts landed at **2.1.6**, verified against `script_by_code` and
  `script_all_codes` rather than taken on trust. It was listed in error.

### Notes

- **The data was gathered and adversarially verified.** Twelve agents in a
  gather-then-verify pipeline; each verifier saw only the claims, not the
  reasoning, and was told to refute rather than confirm. **43 of 44 grammar values
  survived unchanged.** The single dispute was about how to gloss English `-n't` in
  a comment — the verifier confirmed the encoded value (`NEG_PARTICLE`) was right.

- **The verifier caught a real error elsewhere**: Guarani was proposed as
  `Vulnerable` and corrected to `Safe` (~6.5M speakers, co-official in Paraguay,
  full intergenerational transmission). That correction is applied in the 2.3.2
  reference table.

- **The verification design has a hole, and it bit.** The endangerment agent
  returned 50 of 51 languages, silently omitting Korean, and the verifier did not
  notice — a reviewer checking claims only sees what is present, never what is
  absent. Caught by comparing the returned codes against the registry. Recorded in
  `typology-data.md`; any future gather/verify pass needs a completeness check
  independent of the reviewer.

- **Glottocode support is deliberately not implemented.** Glottocodes are opaque
  identifiers (`stan1293`) that can only be looked up, never derived. Generating 51
  from memory would produce plausible-looking fabrications indistinguishable from
  real ones — the worst failure mode for a reference corpus. Filed as a data item
  needing an authoritative source.

- Alignment records **syntactic** alignment. WALS 98A (case marking on full NPs)
  is a separate feature that would code both English and Mandarin as *Neutral*;
  it is not encoded here, and the profiles say so at the point of decision.

### Performance

- **Flat.** A first two-round A/B suggested `phoneme_lookup_ipa` had regressed
  10.5%, which would have been unexplainable — nothing in this release touches
  phoneme lookup. Five rounds dissolved it: the old build's spread is 306-346 ns
  and the new one's 338-341, overlapping distributions where the min-of-N statistic
  had latched onto one lucky low sample. `grammar_by_code_lookup`, the row that
  would actually move, is +0.9%.


## [2.2.4] - 2026-08-27

Last slice of the 2.2.x Phonological Depth line: tone becomes structured data.
Includes one **breaking** change to `phoneme_tones`.

### Breaking

- **phoneme** — `phoneme_tones(inv)` now returns a vec of **Tone records** rather
  than a vec of Chao strings.

  **Migration**: wrap element reads in `tone_letters(t)` to get the old string
  back. Nothing the previous representation carried is lost — the letters are
  preserved verbatim, and the record adds contour, register, endpoint levels and
  features alongside them.

  `phoneme_builder_tones` still *takes* strings and parses them, so the corpus
  stays declarative and the letters remain the single source of truth.

### Added

- **src/tone.cyr** — a new module (wired into `src/main.cyr` and both
  `cyrius.cyml` profiles) parsing Chao tone-letter notation into records:

  ```
  tone_parse(letters)   -> Tone
  tone_contour(t)       -> level / rising / falling / dipping / peaking
  tone_register(t)      -> high / mid / low  (from the mean pitch level)
  tone_start_level(t), tone_end_level(t)     -> the 1-5 Chao scale
  tone_features(t)      -> glottalized (ˀ) | checked (final ʔ)
  tone_letters(t)       -> the original string
  ```

  U+02E5..U+02E9 normalise to 5..1; ˀ (U+02C0) and a final ʔ (U+0294) are recorded
  as features rather than pitch. Unmodelled bytes are skipped rather than rejected,
  so an unknown diacritic degrades the record instead of discarding the tone.

- **tests/tone.tcyr** (163 assertions). Expectations come from each language's
  tonology, not from re-running the parser: Mandarin's 55/35/214/51 must come out
  level/rising/dipping/falling, Vietnamese ngã and nặng must be the glottalised
  pair, the Literary Chinese entering tone must be checked. Plus corpus-wide
  structure — every level on the 1-5 scale, every contour consistent with its own
  endpoints — and a round-trip check that every record returns its original letters.

### Notes

- **Nine languages carry tones, not the five the roadmap listed.** It named `zh`,
  `yo`, `th`, `vi`, `lzh`; **Hausa, Burmese, Somali and Lao** also have tone
  inventories, for 37 tones in total. The existing tonal/non-tonal test in
  `tests/integration.tcyr` had not caught this because those four appeared in
  neither of its two hard-coded lists. The corpus-wide assertion here pins 9 and 37,
  so a tenth cannot be added silently.

- **Parsed, not hand-transcribed** — the same choice `src/features.cyr` made at
  2.2.3, for the same reason: two hand-maintained encodings of one fact drift apart.

- **A data error surfaced and was left alone.** Both Thai and Vietnamese list a
  `˨˩˦` tone, which is the shape of *Mandarin's* third tone. Thai's inventory is
  missing its falling tone `˥˩` and has `˨˩˦` where that should be; Vietnamese's
  huyền should be `˨˩`, not `˨˩˦`. The same wrong value in two unrelated languages
  points at a copy-paste from the Mandarin entry. Correcting tone transcriptions is
  a data change beyond this release, so it is filed in the roadmap rather than made
  silently — the structured records are what made it visible.

- **Adding a module costs 19 test files.** Every test including `src/phoneme.cyr`
  needed `src/tone.cyr` ahead of it, since the builder now calls into it. Worth
  knowing before the next module lands.

### Performance

- **Flat** — no benchmark row moves more than run-to-run noise. Memory for all 51
  inventories goes 139,152 → 147,920 bytes (+8,768) for the 37 tone records and
  their vecs, paid once at build since the constructors have been cached since
  2.1.5.


## [2.2.3] - 2026-08-27

Fourth slice of the 2.2.x Phonological Depth line: the SPE/Hayes distinctive-feature
system, **derived** from the axes 2.2.0-2.2.2 put in place rather than stored. No
struct growth, no breaking changes.

### Added

- **src/features.cyr** — a new module (wired into `src/main.cyr` and both
  `cyrius.cyml` bundle profiles) providing `DistinctiveFeature`, 25 features across
  major class, manner, laryngeal, place and vowel quality:

  ```
  phoneme_df(p, f)        -> 1 (+), 0 (-), (0-1) unspecified
  phoneme_df_present(p)   -> bitmask of features valued +
  phoneme_df_defined(p)   -> bitmask of features specified at all
  df_name(f)              -> "sonorant", "spread glottis", …
  phoneme_minimal_contrast(a, b) -> the single feature two phonemes differ on, or 0
  ```

- **tests/distinctive.tcyr** (195 assertions). The expectations are taken from the
  phonetics literature for segments whose values are not in dispute — /p/ is
  [−sonorant, −continuant, +labial], /s/ is [+strident, +anterior], /θ/ is
  [−strident, +distributed] — rather than re-derived from the same inputs, which
  would have proved nothing. Plus corpus-wide structural checks over all 1,649
  phonemes: `present` is always a subset of `defined`, no obstruent is [+sonorant],
  every nasal is, every consonant is [−syllabic].

### Notes

- **Derived, not stored.** Almost every distinctive feature is a function of manner,
  place, voicing, height, backness, airstream and the secondary-articulation mask.
  Storing them again would double the per-phoneme cost and let the two
  representations drift — an inventory edited to change a manner would keep a stale
  [±sonorant]. Derivation makes that impossible. This is why the roadmap sequenced
  2.2.3 after 2.2.1 and 2.2.2.

- **Three values, not two.** A feature can be +, −, or genuinely unspecified:
  [±anterior] is coronal-only, [±delayed release] applies to stops and affricates,
  and [±ATR] is untranscribed corpus-wide. Collapsing unspecified into − would
  assert things the data does not support. Two cases are left unspecified because
  the literature genuinely splits, not because the data is missing: **trills and
  taps** for [±continuant] (intermittent closure), and **central vowels** for
  [±back] (a two-valued feature cannot carry a three-way contrast, and the
  `Backness` axis already records it exactly).

- **Secondary articulation feeds place.** Palatalized consonants come out [+dorsal,
  +high], labialized ones [+labial, +round], and the Arabic emphatics [+low, +back]
  without being pharyngeal by place — all falling out of 2.2.1's mask rather than
  needing their own data.

- **The derivation allocated, and now does not.** The first implementation used a
  16-byte accumulator per call, which the bump allocator would never reclaim —
  measured at 16,000 bytes per 1,000 reads. That is exactly the defect 2.1.5 swept
  out of the rest of the tree, so it was replaced with a single reused scratch
  record before release. `tests/distinctive.tcyr` asserts that 1,500 consecutive
  feature reads allocate zero bytes.

### Performance

- **Flat**, and **memory unchanged** at 139,152 bytes for all 51 inventories — the
  features are computed on demand and the Phoneme record stays 56 bytes.


## [2.2.2] - 2026-08-27

Third slice of the 2.2.x Phonological Depth line: vowel length, nasalization and
syllabicity become queryable. No struct growth and no breaking changes — the vowel
features reuse the `PhonemeFeature` word 2.2.1 added.

### Added

- **phoneme** — three vowel-side bits on the existing mask: `Nasalized` (2048),
  `Syllabic` (4096), `ExtraShort` (8192). `Long` (256) is **shared** with the
  consonant side rather than duplicated. Construct with `phoneme_vowel_full(...)`
  or `phoneme_builder_vowel_full(...)`; `phoneme_vowel` keeps its arity and
  defaults to no features.

- **inventories** — **118 vowels marked**, carrying 120 feature bits: 110 long
  across 20 languages, 5 nasal (French /ɛ̃ ɑ̃ œ̃ ɔ̃/, Guarani /ɨ̃/), 4 syllabic
  (Sanskrit /r̩ l̩ r̩ː l̩ː/) and 1 extra-short (Vietnamese /ɤ̆/). Sanskrit's /r̩ː l̩ː/
  are the only segments in the corpus carrying two vowel features at once.

- **tests/vowel_features.tcyr** (72 assertions) — per-language cases, corpus
  totals, a `symbol_vowel_feature_agreement` guard matching the consonant one, and
  a `long_is_shared_across_kinds` group that separates the 110 long vowels from
  the 1 long consonant.

### Changed

- **tests/features.tcyr** — `_count_feature` is now kind-aware. Sharing the `Long`
  bit meant an unfiltered count of long segments swept the new vowels into the
  consonant total; the existing assertion caught it (111 where 1 was expected).
  The "no vowel carries a consonant feature" assertion, which 2.2.2 makes false by
  design, is replaced by the stronger claim that no vowel carries a
  *consonant-only* feature — aspiration, laterality and the rest.

### Notes

- **ATR was named in the roadmap and is deliberately not implemented.**
  `tests/vowel_features.tcyr` has an `atr_is_absent` group that pins the reasons so
  the omission is not read as an oversight:
  1. **No corpus segment is transcribed with an ATR/RTR diacritic** (U+0318 /
     U+0319) — the assertion counts zero. Adding an `AdvancedTongueRoot` bit would
     ship a feature nothing sets, the same mistake `Velarized` was kept out of
     2.2.1 to avoid.
  2. **It is not a one-bit change.** Three corpus languages have ATR harmony and
     two have incomplete vowel systems for it: Yoruba's /i e ɛ a ɔ o u/ is the
     classic 7-vowel ATR set and could be marked as it stands, but Wolof lists only
     /i e a o/ and Somali /i e a o u/ — both missing the vowels the contrast needs.
     Marking Yoruba alone would leave ATR present in one language and silently
     absent in two that have it, which is worse than uniformly absent.

  Filed in the roadmap as a data item naming the three languages and what each
  needs, rather than shipped half-done.

### Performance

- **Flat**, and **memory unchanged**: no row moves more than run-to-run noise, and
  building all 51 inventories still takes 139,152 bytes. The vowel features occupy
  bits in the word 2.2.1 already allocated, so unlike the previous two releases
  this one widens nothing.


## [2.2.1] - 2026-08-27

Second slice of the 2.2.x Phonological Depth line: secondary articulation and
phonation as a queryable bitmask, and the lateral-click manner fix 2.2.0 deferred.
No breaking changes — the existing constructors keep their arity.

### Added

- **phoneme** — `PhonemeFeature`, an 11-bit mask on the phoneme record:
  `Aspirated`, `Breathy`, `Palatalized`, `Labialized`, `Pharyngealized`, `Tense`,
  `Devoiced`, `Prenasalized`, `Long`, `Raised`, `Lateral`. Read with
  `phoneme_features(p)` / `phoneme_has_feature(p, f)`; construct with
  `phoneme_consonant_full(...)` or `phoneme_builder_consonant_full(...)`.

  A bitmask rather than a field per feature because they combine freely (Hindi
  /d̪ʱ/ is breathy *and* dental, Zulu /ǁʰ/ aspirated *and* lateral *and* a click),
  several are rare enough that a slot each would be mostly zeroes, and 2.2.3's
  distinctive-feature bundle can extend the same word.

- **inventories** — **190 consonants marked** across the corpus: 76 aspirated,
  16 palatalized, 15 breathy, 6 devoiced, 5 tense, 4 pharyngealized, 4
  prenasalized, 3 labialized, 1 long, 1 raised, plus the laterals.

- **tests/features.tcyr** (104 assertions) — per-language cases written out by
  hand, a `lateral_consistency` check that the `Lateral` bit and the lateral
  manners agree in both directions, corpus totals per feature, and
  `symbol_feature_agreement`, which requires every secondary-articulation
  diacritic in all 1,649 phonemes to match its bit and no vowel to carry a
  consonant feature yet.

### Changed

- **Lateral clicks** — /ǁ ǁʰ ɡǁ/ move from `Manner.LateralFricative` to
  `Manner.Plosive`, with laterality now carried by `PhonemeFeature.Lateral`. A
  click is a stop, not a fricative; the wrong manner was inherited from before
  2.2.0 and left in place because "lateral" had nowhere else to live. It does now.
  /ŋǁ/ keeps `Manner.Nasal` — a nasal click is still nasal — and gains the bit.
- **Phoneme record** is 56 bytes, up from 48.

### Notes

- **The roadmap named five features; the corpus needs nine.** A scan of all 1,193
  consonants found four more that are real and were unrepresentable: **breathy
  voice** (15 segments — the entire Hindi/Bengali/Urdu voiced-aspirate series),
  **pharyngealization** (4 — the Arabic emphatic contrast), **tense/fortis** (5 —
  Korean), and **devoiced sonorants** (6 — Burmese, Icelandic), plus `Raised`
  (Czech /r̝/) and `Lateral`. `Velarized` is deliberately absent: nothing in the
  corpus needs it, and bits are additive.
- **A transcription inconsistency surfaced and was left alone.** Sanskrit spells
  its voiced aspirates /ɡʰ d͡ʑʰ ɖʰ d̪ʰ bʰ/ with U+02B0, so they derive as
  `Aspirated`; Hindi, Bengali and Urdu spell the same historical series with
  U+02B1 and derive as `Breathy`. Phonetically they are breathy in all four.
  Retranscribing Sanskrit is a data change beyond this release, so the corpus is
  marked as written and the discrepancy is documented at the assertion that pins
  it. Filed for a later data pass.
- **The marks were derived mechanically from the IPA symbols**, so
  `symbol_feature_agreement` cannot independently confirm the conversion — it
  would be checking the derivation against itself. Its value is forward-looking:
  an inventory added later with /pʰ/ and no `Aspirated` bit fails. The
  per-language groups are the ones that pin today's data, and those are hand-written.

### Performance

- **Flat.** Interleaved A/B; the only row moving more than 10% is
  `english_phoneme_inventory` at 3 → 5 ns, which is a 2 ns delta on a row already
  at the harness floor — noise, not a regression. The constructors have been cached
  singletons since 2.1.5, so the wider record is paid once at build.
- **Memory**: +8 bytes per phoneme. Building all 51 inventories moves 125,960 →
  139,152 bytes — exactly 1,649 × 8 — and being cached, that is the whole cost for
  a process.


## [2.2.0] - 2026-08-27

First slice of the 2.2.x Phonological Depth line: an **airstream** axis on the
phoneme record, and the corpus reclassified against it. No breaking changes — the
existing constructors keep their arity and default to pulmonic.

### Added

- **phoneme** — `Airstream` enum (`Pulmonic`, `Ejective`, `Implosive`, `Click`) as a
  field on the Phoneme record, with `phoneme_airstream(p)` to read it,
  `phoneme_consonant_airstream(...)` to construct one, and
  `phoneme_builder_consonant_airstream(...)` on the builder. `phoneme_consonant` and
  `phoneme_vowel` are unchanged and set `Pulmonic` implicitly.

- **inventories** — **38 consonants across 7 languages** reclassified with their
  airstream, manner intact:

  | | count | languages |
  |---|---|---|
  | Click | 12 | Zulu |
  | Implosive | 5 | Zulu, Hausa, **Khmer** |
  | Ejective | 21 | Georgian, **Yucatec Maya**, **Amharic**, **Quechua**, Hausa |

  The roadmap named three languages; a scan of all 51 inventories found seven. Khmer,
  Yucatec Maya, Amharic and Quechua were carrying unmarked implosives and ejectives.

- **tests/airstream.tcyr** (119 assertions). Every check asserts manner *and*
  airstream together — a test that only checked the airstream would pass equally
  under the single-enum model this release rejected. Beyond the per-language cases it
  pins corpus totals (12/5/21 across 7 languages), that no vowel is non-pulmonic,
  that every value is inside the enum, and that `phoneme_clone` carries the axis.

  The load-bearing one is `symbol_airstream_agreement`: it walks all 1,649 phonemes
  and requires that any IPA symbol containing a click letter, the ejective apostrophe
  or an implosive hook is marked accordingly — **and that nothing else claims a
  non-pulmonic airstream**. The totals pin today's corpus; this pins the rule, so a
  future inventory added with an unmarked /ɓ/ fails.

### Changed

- **Phoneme record** is 48 bytes, up from 40.

### Notes

- **The roadmap's `Manner` proposal was rejected, deliberately.** It read "Add
  `Click`, `Implosive`, `Ejective` to `Manner`", but those are airstream mechanisms,
  not manners of articulation, and the corpus shows the collision directly: /ŋǀ/ is a
  nasal click, /t͡sʼ/ an ejective affricate, /ǁ/ a lateral click. A single enum records
  one or the other, so the literal change would have erased the manner of 19 of the 38
  phonemes. A separate axis loses nothing and matches how PHOIBLE models it — which
  was the bullet's own stated goal. Roadmap amended.
- **Lateral clicks keep `Manner.LateralFricative`**, which is wrong — a click is a
  stop, not a fricative. It is pre-existing, and correcting it means moving "lateral"
  somewhere else, which is 2.2.1's job. Left alone rather than silently re-adjudicated
  here, and written into the roadmap so it is not forgotten.
- Aspiration is *not* an airstream: /pʰ/, /ǀʰ/ and /t͡sʰ/ are pulmonic, and the tests
  assert that explicitly. Aspiration becomes queryable at 2.2.1.

### Performance

- **Flat.** Interleaved A/B, every row within run-to-run noise. The inventory
  constructors have been cached singletons since 2.1.5, so the wider record is paid
  once at build rather than per call; the near-zero construction rows move by a few ns
  in both directions, which is noise at that magnitude, not a win.
- **Memory**: +8 bytes per phoneme. Building all 51 inventories moves 112,768 →
  125,960 bytes — exactly 1,649 phonemes × 8 — and, being cached, that is the total
  cost for a process, not a per-call one.


## [2.1.6] - 2026-08-27

Roadmap item 2.1.6: define the nine scripts the registry had been naming without
them. This empties the `2.1.x` carry-over list.

### Added

- **script** — nine ISO 15924 scripts that the registry referenced but `script.cyr`
  never defined, so `registry_primary_script` silently returned 0 for nine of the 51
  languages: **Thai** (`Thai`), **Bengali** (`Beng`), **Tamil** (`Taml`),
  **Ethiopic** (`Ethi`), **Hebrew** (`Hebr`), **Georgian** (`Geor`), **Myanmar**
  (`Mymr`), **Khmer** (`Khmr`) and **Lao** (`Laoo`). Each carries its type,
  direction, status, Unicode blocks and languages, and is reachable through
  `script_by_code` and `script_all_codes`. The registry now resolves a primary script
  for all 51 languages; `script_all_codes()` returns 19, up from 10.

  Blocks follow Unicode, main block first, including the supplementary ones a
  single-range definition would have missed: Tamil Supplement (`11FC0`–`11FFF`),
  Ethiopic Supplement / Extended / Extended-A, Hebrew presentation forms
  (`FB1D`–`FB4F`), Georgian Extended and Supplement, Myanmar Extended-A and -B, and
  Khmer Symbols. Hebrew is the only right-to-left addition; Georgian is the only
  alphabet, the other seven are abugidas.

- **tests/script.tcyr** — a `scripts_added_at_2_1_6` group (52 assertions) covering
  each new script's metadata, dispatch reachability, and codepoint boundaries on both
  sides of every block — including the adjacent-block confusions the ranges invite
  (Thai vs Lao at `0E7F`/`0E80`, Bengali vs Devanagari at `097F`).

### Changed

- **tests/invariants.tcyr** — `registry_script_codes_resolve` is **strict** again.
  The Rust original was deliberately tolerant ("Some scripts may not be registered yet
  (Thai, Beng, etc.)") and the 2.1.3 port carried that allowance, pinning the gap at
  exactly 9 unresolved codes. With the nine defined, the assertion now requires every
  registered language to resolve a primary script — a new language whose script is
  undefined fails here rather than returning 0 at runtime.
- **tests/integration.tcyr** — `registry_script_consistency` now expects all 51
  languages to resolve, up from 42.
- **tests/daimon.tcyr** — the agent registration advertises 19 supported scripts.

### Notes

- No performance change: `script_by_code` grew nine `streq` branches on a path that
  has been a cached singleton lookup since 2.1.5, and `script_by_code_lookup` is flat.
- Suite totals: 1,066 → 1,120 assertions across 26 test files.


## [2.1.5] - 2026-08-27

Roadmap item 2.1.5: the three items deferred out of the 2.1.2 hardening sweep.
Includes one **breaking** change to the `-D MCP` surface.

### Breaking

- **mcp** — `varna_translate_ipa` now returns a JSON object,
  `{"scheme":...,"input":...,"output":...}`, where it previously returned the
  transliterated text as a bare string. The other four tools already returned JSON
  objects, so a consumer parsing every success payload got malformed data from
  exactly one tool.

  **Migration**: read the `"output"` field instead of using the payload directly.
  `mcp_result_payload(r)` → parse → `output`.

  The contract is now stated at the top of `src/mcp.cyr` and pinned by
  `tests/mcp_json.tcyr`: **success payloads are JSON objects for all five tools;
  error payloads are plain diagnostic messages, not JSON.** Errors stay unstructured
  deliberately — the result is already tagged, so a caller never has to parse one to
  discover it is an error.

### Changed

- **Pre-built data constructors are now shared singletons.** All 98 zero-argument
  constructors — 51 `phoneme_*` inventories, 10 `script_*`, 11 `grammar_*`,
  10 `swadesh_*`, 2 `translit_*`, 5 `numerals_*`, plus `allophone_english`,
  the three `phonotactics_*`, `dialect_british_english`, `cognate_water` and the
  four `*_all_codes` list builders — build once and return the same pointer to
  every later caller.

  **This is a public-API semantic change.** Do not mutate a returned inventory
  through the `phoneme_builder_*` functions: you would be editing what every other
  caller sees. Use the new `phoneme_clone` for a mutable copy. `registry_all_codes`
  took this same change at 2.1.2; the contract is now uniform and documented at the
  head of the pre-built section in `src/phoneme.cyr`.

- **mcp** — every value written inside a JSON string literal now goes through
  `_mcp_append_json`, which escapes `"`, `\`, the five short-form control escapes
  and any other C0 byte as `\u00XX` (RFC 8259). Nothing in the current data needs
  an escape, which was exactly the problem: the invariant was unenforced and one new
  field away from emitting malformed JSON.

### Added

- **phoneme** — `phoneme_clone(inv)` returns an independent inventory that is safe
  to mutate. The copy is shallow in the same sense `dialect_apply` has always been:
  the vec, stress and tone slots are fresh, the immutable Phoneme records are shared.
- **tests/caching.tcyr** (54 assertions) — pins the singleton contract: pointer
  identity across repeat calls and through `registry_phonemes` / `script_by_code` /
  `grammar_by_code` / `swadesh_by_code` / `registry_primary_script`, that distinct
  constructors stay distinct objects, that the data is unchanged and does not
  accumulate on re-read, that `phoneme_clone` isolates mutations from the shared
  original, and that `dialect_apply` still leaves its parent untouched now that the
  parent is shared.
- **tests/mcp_json.tcyr** — a `json_escaping` group feeding a quote, a backslash, a
  newline, a tab and a raw `0x01` through `varna_translate_ipa` and checking the
  payload is still valid JSON with each byte escaped correctly.

### Performance

Interleaved A/B in one session (`cyrius bench`, min ns):

- **english_phoneme_inventory** 1,252 → 0 ns, **sanskrit** 1,664 → 6 ns, **greek**
  858 → 6 ns. The 0 is below the harness floor rather than literally free — the call
  is now a load and a compare, which the optimizer can hoist out of the loop.
- **registry_phonemes_lookup** 1,712 → 52 ns (**-97%**);
  **swadesh_by_code_lookup** 1,688 → 110 ns (**-93%**);
  **script_by_code_lookup** 287 → 102 ns (**-65%**).
- **numeral_value_of_char** 302 → 198 ns. Not attributable — the benchmark pre-builds
  its table, so nothing in this change should touch it; most likely allocation
  locality. Reported, not claimed.
- All other rows flat within noise.

**The leak is gone.** Measured directly off the bump allocator's `_heap_used`
counter over 1,000 warm calls, bytes handed out per call:

| call | before | after |
|---|---|---|
| `registry_phonemes("en")` | 2,400 B | 0 B |
| `phoneme_english()` | 2,400 B | 0 B |
| `swadesh_by_code("es")` | 1,624 B | 0 B |
| `script_by_code("Deva")` | 400 B | 0 B |

The allocator never frees, so those were permanent: a consumer polling the registry
once a second leaked ~200 MB a day.


## [2.1.4] - 2026-08-27

Roadmap item 2.1.4: remove the frozen v1.x Rust crate. The tree is single-language
again. No source behaviour changed — the only edits to `src/` are comment rewrites —
so benchmarks are unmoved and no new `bench-history.csv` row was recorded.

### Removed

- **`rust-old/`** — the frozen v1.x Rust crate: 35 files, 8,386 lines, 484K. Deleted per
  [ADR 0002](docs/adr/0002-remove-the-rust-old-archive.md), which supersedes the
  "freeze the Rust source" half of [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md).

  It was kept as the parity oracle because the 2.0.0 port was source-level
  reimplementation, not translation. Two things made removal decidable: the API port was
  verified complete at 2.1.3 (19 modules 1:1, 150/150 `pub fn`, 48/48 inventories matching
  by name, and nothing under `src/` or `tests/` ever compiled against it), and the four
  standalone suites under `rust-old/tests/` — the real dependency, 147 tests — were ported
  in the same release. Recoverable from git tags `1.0.0`–`2.1.3`:
  `git show 2.1.3:rust-old/src/lib.rs`. Recoverability was checked against tags `2.0.0`
  and `2.1.3` before deleting.
- **`.gitignore`** — the `/rust-old/target/` entry, and the stale "Cyrius port" heading
  above it.

### Changed

- **Provenance comments** — 43 references across 42 files in `src/` and `tests/` rewritten
  from `rust-old/src/X.rs` to `v1.x Rust src/X.rs`. The provenance is worth keeping; a path
  that no longer resolves is not. Three bare-form references (`rust-old/` with no file,
  `rust-old inventory_test!`, `rust-old mod.rs sample_lexicon`) were reworded individually
  rather than pattern-matched.
- **docs/adr/0001-port-from-rust-to-cyrius.md** — status amended to note the supersession,
  with a dated amendment block. The body is left as written: an ADR records what was
  decided at the time, so its `rust-old/` references stay.
- **README.md**, **SECURITY.md**, **CLAUDE.md**, **docs/guides/getting-started.md**,
  **docs/development/state.md** — `rust-old/` mentions replaced with the tag-recovery
  route. `getting-started.md` lost its "cross-check parity against `rust-old/`" step
  (the oracle it named is gone; the behaviour now lives in the ported suites) and gained
  `dist/` in the layout list, which was never described.
- **docs/benchmarks-rust-vs-cyrius.md** — **kept**, deliberately. Its criterion figures
  are the only surviving record of v1.x performance and are not reproducible from anything
  still in the tree. Annotated with where the source went and how to recover it.

### Notes

- `git rm` was used, so the 35 deletions are staged. Nothing is committed.
- The "93 references across 49 files" figure quoted in the 2.1.3 roadmap counted the
  generated `dist/*.cyr` bundles alongside their sources. The real hand-edited count was
  43 across 42 files; the bundles picked up the rewrite when `cyrius distlib` regenerated.


## [2.1.3] - 2026-08-27

Roadmap item 2.1.3: port the four standalone Rust test suites the 2.0.0 port left
behind. Test-only — no source changes, so benchmarks are unmoved and no new
`bench-history.csv` row was recorded.

The 2.0.0 port carried every module's public API and its in-module unit tests, but
not the 147 tests under `rust-old/tests/`. That gap is why `rust-old/` could not be
deleted: removing it would have destroyed the record of what those tests asserted.
It is now closed, which unblocks 2.1.4.

### Added

- **tests/invariants.tcyr** (39 assertions) — port of `rust-old/tests/invariants.rs`
  (33 tests). Structural guarantees across the whole dataset: code uniqueness for
  languages/scripts/grammar/Swadesh, consonant+vowel totals, no duplicate or empty
  IPA symbols, inventory codes matching their registry key, tones and `Tonal` stress
  implying each other, well-formed Unicode ranges, 25-entry Swadesh lists covering
  indices 1-25, numeral tables without duplicates, decimal digit runs, and
  registered dialect parents and cognate languages.
- **tests/adversarial.tcyr** (74 assertions) — port of `rust-old/tests/adversarial.rs`
  (54 tests): empty strings, 1000-character codes, emoji, BOM, zero-width space,
  IPA lookalikes (`S` vs `ʃ`, ASCII `g` vs `ɡ`), case-sensitive ISO 15924 codes,
  codepoint boundaries either side of every script range, `u32::MAX`, empty and
  zero-capacity builders, `most_frequent(0)` and `most_frequent(1_000_000)`,
  unmapped and mixed-validity numeral strings, and dialect idempotence. **This is
  the suite whose absence let the 2.1.2 heap overflows survive** — it already
  contained `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`.
- **tests/integration.tcyr** (35 assertions) — the 18 behavioural tests from
  `rust-old/tests/integration.rs`: phoneme-kind classification, constructor field
  round-trips, registry/script consistency, the allophone/inventory boundary
  (`/t/` is an English phoneme, its intervocalic tap `[ɾ]` is not), Greek isopsephy
  of θεος, the RP overlay, cognate/registry cross-checks, Swadesh index 23 glossing
  as "water" in every list, and the tonal/non-tonal split.
- **tests/mcp_json.tcyr** (28 assertions) — stands in for
  `rust-old/tests/serde_roundtrip.rs` (27 tests), which cannot port directly: there
  is no serde in Cyrius and no deserializer to round-trip against. It checks the
  property the roundtrips were protecting — that every payload the library emits is
  structurally valid JSON — across 51 phoneme payloads, every script and grammar
  payload, and 204 compare payloads, using a structural validator that is itself
  self-checked against eleven malformed inputs.

### Changed

- **docs/development/roadmap.md** — the pre-port `1.1.0`-`1.5.0` tiers are
  renumbered `2.2.0`-`2.6.0` and the section retitled; the old numbers predated the
  2.0.0 port and no longer tracked the shipping version. 2.1.3 is removed from the
  carry-over list and 2.1.4 (`rust-old/` removal) is no longer gated.
- **docs/development/roadmap.md** — reduced to open work only. The `Completed`
  section (2.1.2 back through the 0.x Rust-crate line) is dropped; release history
  lives in this file and the git tags. Added a `2.1.x — Carry-over` section for
  work identified but not taken: 2.1.3 port the Rust integration suites, 2.1.4
  remove `rust-old/` (gated on 2.1.3), 2.1.5 the deferments from the 2.1.2
  hardening sweep, 2.1.6 script-registry completeness. The retired `v1.0 Criteria`
  checklist left one unmet item (shabda/shabdakosh consumption), now under
  `Downstream`; its "full JSON roundtrip tests for all public types" line was
  inaccurate post-port and is corrected there.
- **docs/development/state.md**, **CLAUDE.md** — record the port-completeness
  verdict, and correct CLAUDE.md's claim that `rust-old/` is "gitignored from CI"
  (it is tracked; only `rust-old/target/` is ignored).

### Verified

- **Rust → Cyrius port is complete at the API level.** 19 Rust modules map 1:1 to
  19 Cyrius modules (plus `util.cyr`); all 150 Rust `pub fn` have Cyrius
  counterparts, checked name by name; the 48 language inventories match exactly.
  Nothing in `src/` or `tests/` compiles against `rust-old/` — the 93 references
  across 49 files are provenance comments.
- **The gap is test coverage, not code.** The four suites under `rust-old/tests/`
  — `invariants.rs` (33), `adversarial.rs` (54), `integration.rs` (33),
  `serde_roundtrip.rs` (27) — were never ported. `adversarial.rs` already contains
  `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`: the class of test that would have caught the 2.1.2
  heap overflows. `rust-old/` is therefore **not yet safe to delete** — porting
  those suites first is scheduled as 2.1.3.
- **26 of 27 checkable Rust invariants hold** against current Cyrius data
  (uniqueness, consonant+vowel totals, no duplicate/empty IPA symbols, well-formed
  Unicode ranges, 25-entry Swadesh lists with indices 1-25, numeral tables,
  registered dialect parents and cognate languages). The 27th,
  `registry_script_codes_resolve`, finds 9 languages whose primary script is
  unregistered — **not a port regression**: the Rust invariant explicitly tolerated
  the same gap ("Some scripts may not be registered yet (Thai, Beng, etc.)").
  Tracked as 2.1.6.


### Notes

- **Four Rust tests do not port, and are documented inline rather than dropped
  silently.** `registry_info("en\0")` and `transliterate("\0")` tested that a Rust
  `String` may carry an interior NUL; a Cyrius cstring ends at the first NUL, so the
  property does not exist here. The three `error_display_*` tests formatted a
  payload into `VarnaError`, which in Cyrius is a plain integer code with static
  messages — the concern behind them (an unbounded caller string reaching a
  formatter) lives in `_tool_err2` and is covered by `tests/hardening_surfaces.tcyr`.
- **All 26 checkable invariants hold.** The 27th, `registry_script_codes_resolve`,
  is ported in the tolerant form the Rust original used and pins the current gap
  exactly: 9 of 51 languages name an unregistered primary script (Thai, Bengali,
  Tamil, Amharic, Hebrew, Georgian, Burmese, Khmer, Lao). Not a port regression —
  the Rust invariant carried the same allowance in a comment. Roadmap 2.1.6 adds the
  scripts, at which point the assertion can be tightened.
- Suite totals: 21 → 25 test files, 652 → 1,005 assertions, reference coverage
  89% → 94%.


## [2.1.2] - 2026-08-27

Hardening release from a P(-1) scaffold sweep. Six memory-safety defects fixed —
two of them heap overflows reachable from the `-D MCP` and `-D HOOSH` surfaces
with caller-controlled length — plus the allocation-churn and lookup-complexity
work the audit turned up. No public API or linguistic data changed.

### Security

- **mcp** — `_tool_err2` rendered `"<prefix><value>"` into a fixed `alloc(256)`
  with no bound on `value`, which is an unvalidated tool parameter. A 400-byte
  `language` / `code` / `scheme` / `lang1` / tool name wrote ~418 bytes into the
  256-byte allocation — a heap overflow with both length and contents chosen by
  the caller. Reachable from all five tools plus the unknown-tool path
  (6 call sites). Every append in the file now takes the buffer capacity and
  truncates; `tests/hardening_surfaces.tcyr` fails on the pre-fix code at all six.
- **hoosh** — `hoosh_answer_from_data` interpolated the caller's `ipa` string into
  a fixed `alloc(256)`. On the not-in-inventory branch that string is never matched
  against anything first, so its length was entirely the caller's choice — the same
  overflow. Bounded the same way.
- **transliteration** — `translit_apply` walked the input with `_utf8_len`, which
  classifies a lead byte without checking that its continuation bytes are present.
  A truncated codepoint at the tail (a lone `0xF0`) advanced the cursor 4 bytes
  with 1 byte left, so the following `memcpy` read up to 3 bytes past the end of
  the input. Reachable through the `varna_translate_ipa` MCP tool.
- **numerals** — `numerals_string_value` had the identical over-read, and is
  reachable through daimon's `numeral_value` capability.
- **util** — new `_utf8_step(s, p, n)` clamps every stride to the bytes actually
  present and never returns 0; it is now the only way the tree walks UTF-8.
  `_utf8_len` is kept and documented as lead-byte classification only.
- **transliteration** — `translit_apply` sized its output at `inlen * 4`, which
  held only because both bundled tables happen to contract. A table mapping a
  short source to a longer target overflowed the buffer, and nothing
  bounds-checked the writes. Output is now sized from the table's longest target
  (`_translit_max_target`, memoised and invalidated by `_tt_add`) and every write
  is checked against the capacity regardless.

### Changed

- **registry** — `registry_all_codes` caches its vec instead of rebuilding a
  51-element one per call. **The result is now shared and must be treated as
  read-only.** The old behaviour leaked a vec per call, since the bump allocator
  never frees.
- **phoneme** — documented that `phoneme_builder_with_capacity`'s `cap` argument
  is ignored: `lib/vec.cyr` exposes no capacity-taking constructor. Kept in the
  signature because the bundle is a published consumer API.

### Added

- **tests** — `tests/hardening.tcyr` (26 assertions) and
  `tests/hardening_surfaces.tcyr` (28) pin every fix above. Both were checked
  against the pre-fix code: the surface tests fail 7 assertions and the
  expanding-table test fails 2, so they are regression guards rather than
  restatements of current behaviour.
- **tests** — `registry_table_consistency` ties `registry_phonemes` to
  `_registry_build`. The two list the same 51 codes in different forms with
  nothing connecting them, so a language could be registered with no inventory
  and only surface downstream.
- **tests** — coverage sweep over the accessor surface `cyrius coverage` reported as
  unreferenced. Reference coverage 204/278 fns (73%) → 249/278 (89%), clearing the 80%
  floor in CLAUDE.md; `swadesh`, `syllable`, `lexicon`, `allophone`, `numerals`,
  `dialect` and `error` are now at 100%, and `error.cyr` went from no referenced fn at
  all to covered. 222 new assertions, 17 → 19 test files (21 with the two above).
  - **swadesh** — `tests/swadesh.tcyr` (+100): all ten list builders called directly
    instead of only through the `swadesh_by_code` dispatch table, each checked for
    code, 25 entries and the first/last entry's word, IPA, gloss and Swadesh index;
    `swadesh_entry` / `_word` / `_ipa` / `_gloss` pinned against the shared gloss set.
  - **syllable** — `tests/syllable.tcyr` (+42): `SyllableTemplate` accessors read
    directly and cross-checked against the delegating `phonotactics_*` ones;
    `phonotactics_language_code` / `_syllable` / `_constraints`; `constraint_kind` /
    `_position` / `_sequences` / `_description` over the English, Japanese and
    Sanskrit profiles; and an open-syllable (`max_coda 0`) template for the coda
    thresholds no pre-built profile reaches.
  - **lexicon** — new `tests/lexicon.tcyr` (24): `LexEntry` field accessors, the
    `(0-1)` None sentinel for `frequency_rank` / `swadesh_index`, `lexicon_entries`
    insertion order, and `lexicon_find_gloss` vs `lexicon_find` on a non-English
    lexicon where native word and English gloss cannot agree.
  - **numerals** — `tests/numerals.tcyr` (+19): `numerals_script_code` / `_name` /
    `_mappings` for all five systems, pinning each Unicode script tag and table size.
  - **allophone** — `tests/allophone.tcyr` (+11): `allophone_rule_phoneme` /
    `_allophone` / `_environment` / `_obligatory` — the environment and obligatoriness
    that `allophone_realize` collapses away.
  - **dialect** — `tests/dialect.tcyr` (+11): `dialect_code` / `_name` / `_added` /
    `_removed`, and the RP intervocalic-/t/ override read through the rule accessors.
  - **error** — new `tests/error.tcyr` (15): `varna_error_str` for every `VarnaError`
    code plus the out-of-range fallback, so an unknown sentinel still renders.

### Performance

Measured by interleaving old and new builds in one session (`cyrius bench`, min
ns) — a straight before/after against an earlier baseline showed a phantom
15-25% regression across untouched rows that was pure machine drift.

- **registry_all_codes_iter** — 42,499 → 3,310 ns (**-92%**, 12.8x).
  `registry_info` was a linear `streq` scan over 51 entries, so iterating the
  registry cost ~1,300 string compares; it is now a lazily built `map_get`.
- **transliterate_greek_word** — 39,636 → 19,351 ns (**-51%**). `translit_apply`
  allocated a `lens` vec plus up to four NUL-terminated candidate buffers *per
  input grapheme*; matching is now done in place against the table literals.
- **numeral_string_value_word** — 1,591 → 969 ns (**-39%**); the per-character
  allocation is gone the same way.
- **numeral_value_of_char** — 502 → 310 ns (**-38%**) and
  **transliterate_devanagari_char** — 916 → 575 ns (**-37%**): one `strlen` of the
  needle per call instead of one per table entry.
- Every other row is flat within its run-to-run noise. `script_contains_codepoint`
  reads 16 → 14 ns but that is inside its own ±14% spread and nothing in
  `script.cyr` changed — not claimed as a win.


## [2.1.1] - 2026-08-27

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.4.69` → `6.5.35` (`cyrius.cyml [package].cyrius`);
  CI and the release workflow derive their install version from the pin, so the whole
  pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.5.35 snapshot.
  The closure is the same 29 modules as at 2.1.0, but 20 of them changed content —
  most substantially `bayan` (TOML/JSON parser, ~10.8k changed lines), `syscalls_x86_64_agnos`,
  `alloc`, `bench`, `io`, `vec`, and the per-arch `syscalls_*` set. Upstream added
  `async_macos`/`thread_macos` to the snapshot; neither is in varna's closure.
- **cyrius.lock** — regenerated at 6.5.35; `cyrius deps --verify` is clean (29 verified,
  0 failed). Resolved with `cyrius deps`, whose include-graph walk is the authority:
  6.5.35's new `cyrius lib sync` matches on declared `[deps].stdlib` names instead and
  produces a *different* 29-module set — it adds the unreferenced `hashmap_fast.cyr`
  and drops `atomic.cyr`, which `lib/alloc.cyr` includes transitively and the build
  needs. Locking that set would break a fresh `cyrius deps --verify`.
- **bote** — the commented-out `[deps.bote]` recipe in `cyrius.cyml` (inert; `-D MCP`
  hand-builds its JSON today) refreshed from the stale `tag = "2.7.6"` to `3.3.7`,
  which pins the same Cyrius 6.5.35 toolchain. Its companion notes were corrected
  against the real `dist/bote-core.deps` sidecar: the profile is 12 modules (not 9),
  and the only stdlib leaf varna does not already declare is `chrono` — not the
  `ct`/`keccak`/`random`/`thread`/`thread_local`/`sigil`/`freelist` set the old
  comment listed.
- **daimon** — agent-registration version string bumped to `2.1.1` (`src/daimon.cyr`).
- **dist** — bundles regenerated at `2.1.1`. `cyrius distlib` now also emits a `.deps`
  stdlib-leaf sidecar per profile, and `cyrius distlib --all` writes the `core` profile
  bundle, so three files join the tracked `dist/`: `varna.deps` (16 leaves),
  `varna-core.cyr`, and `varna-core.deps` (4 leaves — `string`, `alloc`, `vec`,
  `hashmap`). `[lib]` and `[lib.core]` declare identical module lists, so the two
  bundle bodies are byte-identical; the sidecar is what distinguishes them.
- **scripts/check.sh** — the gate now runs `cyrius deps --verify` after `cyrius deps`,
  so a lock that does not match the snapshot it claims to pin fails locally and in CI
  instead of shipping (see *Fixed* below).
- **scripts/bench-history.sh** — now records the toolchain version and the harness's
  measured timer floor in `BENCHMARKS.md`, plus a note marking the pre-/post-6.5.19
  measurement discontinuity. On a parse failure it echoes the captured output instead
  of failing silently.

### Fixed

- **cyrius.lock** — the lock shipped at 2.1.0 recorded a wrong hash for
  `lib/syscalls_x86_64_agnos.cyr` (`c5e63ab…` against the 6.4.69 snapshot's actual
  `e53e129…`), so `cyrius deps --verify` reported `28 verified, 1 failed` on a clean
  checkout. CI never caught it because `scripts/check.sh` ran plain `cyrius deps`,
  not `--verify`. Regenerating at 6.5.35 clears it, and the gate now runs `--verify`
  so a stale lock cannot ship again.
- **Build warnings** — the gate is now warning-free. 6.4.69's vendored `lib/bayan.cyr`
  emitted three `assigning non-pointer to typed pointer` warnings on *every* compile
  (`_toml_parse_str`/`_toml_parse_multiline_q` were declared `: i64` but assigned into
  a `Str`); 6.5.35 declares them `: Str`. The `toolchain drift` warning is gone too,
  now that the pin matches the installed `cycc`.

### Performance

- **Stdlib refresh is performance-neutral.** Holding the benchmark harness constant at
  6.5.35 and swapping only the other 28 vendored modules, every benchmark lands within
  run-to-run noise (min ns, 6.4.69 → 6.5.35 stdlib): `english_phoneme_inventory`
  1272 → 1341, `registry_all_codes_iter` 42306 → 42820, `transliterate_greek_word`
  39686 → 40632, `dialect_apply_overlay` 2906 → 3215. Binary size is unchanged at
  597,976 bytes for the default build.
- **The recorded numbers moved, but the instrument moved — not the code.** Cyrius
  6.5.19 rewrote `lib/bench.cyr`: `bench_run` used to wrap a clock pair around every
  iteration, flooring all 18 varna benchmarks at roughly two clock reads (the
  ~419ns/489ns/907ns plateaus visible throughout the old history). It now sizes its own
  batches and subtracts a *measured* per-host clock cost (1.409us on the recording
  host). Sub-microsecond rows finally resolve — `script_contains_codepoint` reads 14ns
  and `allophone_realize` 34ns where both previously bottomed out at 419ns — and the
  heavier rows shift because they are no longer inflated by an unsubtracted floor.
  Rows in `bench-history.csv` on either side of 2026-08-27 are **not** comparable;
  the floor is a property of the host clocksource and moves between reboots. See
  `BENCHMARKS.md`.

## [2.1.0] - 2026-07-21

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.2.12` → `6.4.69` (`cyrius.cyml [package].cyrius`);
  CI derives its install version from the pin, so the whole pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.4.69 snapshot. Upstream
  dropped `agnosys`; `async_win`/`protobuf`/`yantra` are new (none used by varna).
- **cyrius.lock** — regenerated at 6.4.69. Now pins the exact 29-module dependency closure a
  fresh `cyrius deps` resolves, so `cyrius deps --verify` is clean (29 verified, 0 failed).
  Removed 8 stale entries that were never actually resolved or linked by any build config
  (`agnosys`, `bote-core`, `libro`, `majra`, `log`, `sakshi`, `sigil`, `patra`) and added the
  benchmark-harness dependency `bench`.
- **daimon** — agent-registration version string bumped to `2.1.0` (`src/daimon.cyr`).
- **dist** — `dist/varna.cyr` bundle regenerated (header `# Version: 2.1.0`).

### Fixed

- **scripts/bench-history.sh** — `to_ns()` now parses the fractional-unit benchmark
  output Cyrius ≥6.3 emits (`1.396us`); the old converter only stripped leading integer
  digits, so decimal timings were written to `bench-history.csv` unscaled (`1.396` instead
  of `1396`). Results are rounded to integer nanoseconds, matching the existing history.

### Performance

- No regressions from the toolchain move (min ns, 6.2.12 → 6.4.69): `english_phoneme_inventory`
  2000 → 1396, `transliterate_greek_word` 38000 → 36527, `registry_all_codes_iter` 37000 → 36666;
  all other benchmarks within noise. See `bench-history.csv` / `BENCHMARKS.md`.

## [2.0.0] - 2026-06-16

Ported from a Rust crate to the [Cyrius](https://github.com/MacCracken/cyrius)
systems language. See [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md) for the
full rationale. The `content`/data model and the public surface are preserved at
parity; the implementation language, build system, and dependency stack changed.

### Breaking

- **Language** — varna is now a Cyrius library (`dist/varna.cyr`), not a Rust crate.
  The v1.x Rust source is frozen in `rust-old/`. Consumers depend on varna via a
  `[deps.varna]` block in `cyrius.cyml` (`modules = ["dist/varna.cyr"]`), not Cargo.
- **API shape** — the Rust module-path API (`varna::phoneme::english()`) is re-exposed
  as snake-case free functions (`phoneme_english()`); `Option`/`unwrap` access becomes
  sentinel-return checks; `Result` errors use native tagged `enum`s.
- **Feature flags → build defines** — the Cargo `std`/`logging`/`mcp`/`daimon`/`hoosh`/`full`
  features are replaced by `-D LOGGING`/`-D MCP`/`-D DAIMON`/`-D HOOSH` passed to
  `cyrius build`. There is no `std`/`no_std` split.
- **Renames for project-name consistency** — `LIPI_LOG` → `VARNA_LOG`; `LipiError` →
  `VarnaError`; MCP tools `lipi_*` → `varna_*` (`varna_phonemes`, `varna_script`,
  `varna_grammar`, `varna_translate_ipa`, `varna_compare`); `ResponseSource::LipiData`
  → `VarnaData`.

### Changed

- **Dependency stack** — every external Rust dependency now resolves to the Cyrius
  stdlib or a language builtin:
  - `serde` + `serde_json` → `bayan` (`#derive(Serialize)` + `json_parse`/`json_build`)
  - `thiserror` → native `enum` + `Result<T,E>` (`lib/result.cyr`, `lib/tagged.cyr`)
  - `tracing` + `tracing-subscriber` → `lib/log.cyr` + `lib/sakshi.cyr` (env-filter wired by hand)
  - `criterion` → `cyrius bench` + `lib/bench.cyr`
  - Optional `bote` (MCP) → consumed as the `dist/bote-core.cyr` git dep under `-D MCP`
- **String model** — `Cow<'static, str>` fields become `'static` `str` literals (Cyrius
  has no `Cow`); the builder still constructs inventories at runtime.
- **Enums** — `#[non_exhaustive]` enums become Cyrius tagged enums (additive variants).
- **Build + test layout** — `Cargo.toml`/`Cargo.lock` → `cyrius.cyml`/`cyrius.lock`;
  `benches/benchmarks.rs` (criterion) → `benches/*.bcyr`; `tests/integration.rs` →
  `tests/tcyr/*.tcyr`; `make check` (cargo fmt/clippy/test/audit) → `cyrius audit`.

### Removed

- The Rust toolchain config (`rust-toolchain.toml`), `deny.toml`, and the Cargo
  feature system. `cargo doc` browsable API reference (varna's surface is a folded
  `.cyr` bundle, navigable in `src/`).
- The `std`/`no_std`/`alloc` feature distinction — Cyrius emits a single artifact.

### Performance

- Baselined 18 benchmarks via `cyrius bench tests/varna.bcyr` (`bench-history.csv` /
  `BENCHMARKS.md`). Representative per-op `min` figures: phoneme lookup 419ns,
  `script_by_code` 838ns, grammar lookup 419ns, English inventory build 2µs, registry
  iteration over 51 languages 40µs.

### Note

The source-level port is **complete**: all 19 modules (15 core + 4 `-D` surfaces) are
reimplemented as `src/*.cyr`, verified by **526 parity assertions** (`cyrius tests`)
against the frozen `rust-old/` oracle plus a 7-agent parity audit (0 unported public
functions). Documented divergences from the oracle: sentinel returns for `Option`/`Result`,
integer-tagged enums, hand-built JSON (`mcp` uses a lightweight `ToolDef`, no `bote`/`bayan`),
`hoosh` confidence as per-mille, and the name-colliding enum variants `WordOrder.Free` /
`VarietyKind.Historical` / `PhonemeClass` omitted. `daimon`/`hoosh` remain Cyrius binaries
with no consumable library bundle, so those define-gated surfaces only toggle JSON output.

## [1.0.0] - 2026-03-31

### Added

- **phoneme::inventories** — 24 additional language inventories reaching 51 total:
  - European: Italian (23C+7V), Dutch (19C+13V), Polish (29C+6V), Czech (25C+10V), Hungarian (25C+14V), Romanian (21C+7V), Icelandic (22C+8V)
  - South Asian: Bengali (30C+7V), Tamil (19C+10V), Urdu (37C+10V)
  - African: Amharic (27C+7V), Hausa (25C+10V), Somali (22C+10V), Wolof (25C+8V)
  - Southeast Asian: Indonesian (18C+6V), Burmese (30C+7V), Khmer (24C+18V), Lao (20C+9V)
  - Middle East/Caucasus: Persian (23C+6V), Hebrew (23C+5V), Georgian (28C+5V)
  - Americas: Quechua (24C+3V), Guarani (16C+12V)
  - Central Asian: Mongolian (21C+14V)
- **grammar** — English grammar profile added (`grammar::by_code("en")`)
- **docs** — Usage guide (docs/guides/usage.md) with 15 sections and code examples
- **docs** — Architecture overview fully updated for all modules and 51 languages

### Changed

- **registry** — 51 languages registered (up from 27)
- v1.0 criteria: test coverage measured at 98.53%, all documentation complete

## [0.6.0] - 2026-03-31

### Added

- **mcp** — MCP tool definitions (feature-gated `mcp`): `lipi_phonemes`, `lipi_script`, `lipi_grammar`, `lipi_translate_ipa`, `lipi_compare`. Tool registry with `tool_definitions()`, `invoke()` dispatcher, JSON-serializable `ToolResult`. Language comparison computes shared/unique phonemes and grammar differences
- **daimon** — Agent registration for AGNOS framework (feature-gated `daimon`): `AgentRegistration` with 6 capabilities, dynamic language/script coverage from registry. Version auto-synced from Cargo.toml
- **hoosh** — LLM query interface (feature-gated `hoosh`): `LanguageQuery` enum (5 query types), `QueryResponse`, `ResponseSource`. `answer_from_data()` handles phoneme/comparison queries without LLM, returns `None` for queries requiring inference
- Feature gates: `mcp`, `daimon`, `hoosh` (all included in `full`)

## [0.5.0] - 2026-03-31

### Added

- **phoneme::inventories** — 9 core language inventories: Mandarin (21C+7V), Hindi (34C+10V), Japanese (20C+5V), Spanish (23C+5V), French (21C+16V), German (23C+16V), Russian (36C+6V), Korean (19C+7V), Portuguese (23C+14V)
- **grammar** — Pre-built grammar profiles for 10 core languages with `by_code()` lookup and `all_codes()`. Covers morphology, word order, case count, gender, dual number, classifiers
- **lexicon::swadesh** — Swadesh-25 starter word lists for 10 core languages (250 entries total) with IPA transcription, part of speech, and Swadesh index. `by_code()` lookup
- **registry** — 27 languages registered (up from 18)

## [0.4.1] - 2026-03-31

### Added

- **script** — `[S]` Cuneiform script metadata (Xsux) with Sumerian/Akkadian language tags and Unicode ranges
- **script** — `[S]` Egyptian hieroglyphic script metadata (Egyp) with Unicode ranges
- **script** — `ScriptStatus` enum (Living, Limited, Historical) and `attestation` period field on all scripts
- **script::numerals** — `[S]` Babylonian sexagesimal numeral system (base-60, cuneiform digits)
- **script::numerals** — `[S]` Egyptian hieroglyphic numeral system (additive decimal, powers of 10)
- **script::numerals** — `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- **phoneme::inventories** — 4 classical/liturgical language inventories:
  - **Latin** (la) — 18C + 10V, labialized velars
  - **Classical Arabic** (ar) — 28C + 6V, pharyngeals, emphatics, uvular
  - **Koine Greek** (grc) — 17C + 5V, pitch accent
  - **Literary Chinese** (lzh) — 27C + 12V, 4-tone Middle Chinese reconstruction
- **registry** — 18 languages registered (up from 14), 10 scripts (up from 8)

## [0.4.0] - 2026-03-31

### Added

- **dialect** — Language variety support: `LanguageVariety`, `VarietyKind` (Regional, NationalStandard, Historical, Sociolect, Creole). Phoneme add/remove overlays and allophone overrides. British English (RP) and Egyptian Arabic pre-built
- **lexicon::cognate** — Cognate detection types: `CognateSet` with proto-form and cross-language entries, `Etymology` with `BorrowingType` (Loanword, Calque, SemanticLoan, Inherited). PIE "water" cognate set pre-built
- **phoneme::inventories** — 11 new language inventories across 6 language families:
  - `[S]` **Yucatec Maya** (yua) — 21C + 10V, ejective consonants for Mayan calendar validation
  - **Swahili** (sw) — 26C + 5V, **Yoruba** (yo) — 18C + 7V (3-tone), **Zulu** (zu) — 42C + 5V (clicks)
  - **Thai** (th) — 21C + 9V (5-tone), **Vietnamese** (vi) — 22C + 11V (6-tone), **Tagalog** (tl) — 18C + 5V
  - **Turkish** (tr) — 20C + 8V (vowel harmony), **Finnish** (fi) — 17C + 16V (short+long)
  - **Hawaiian** (haw) — 8C + 10V (minimal inventory), **Nahuatl** (nah) — 16C + 8V (lateral affricate)
- **registry** — 14 languages registered (up from 3)

## [0.3.0] - 2026-03-31

### Added

- **phoneme::allophone** — Allophone rule system: `AllophoneRuleSet`, `AllophoneRule`, `Environment`, `PhonemeClass`. Context-dependent sound variation with `rules_for()` and `realize()` lookup. English (GA) rules included (aspiration, flapping, dark-l)
- **phoneme::syllable** — Syllable structure templates: `SyllableTemplate` with max onset/coda, `Phonotactics` with `PhonotacticConstraint`. English, Sanskrit, and Japanese profiles included
- **script::transliteration** — `[S]` Bidirectional transliteration tables: `TransliterationTable` with `transliterate()`, `transliterate_char()`, and `reverse_map()`. Devanagari↔IAST and Greek↔Beta Code tables included
- **script::numerals** — `[S]` Script-to-numeral mapping: `NumeralSystem` with `value_of()`, `char_for()`, and `string_value()`. Devanagari decimal digits and Greek isopsephy included
- `Hash` derive on `Phoneme`, `PhonemeKind`, `Morphology`, `WordOrder`, `Direction`
- `debug_assert` for duplicate IPA detection in `PhonemeInventoryBuilder::build()`

### Changed

- **script** — Kana script name corrected from "Katakana" to "Kana (Hiragana + Katakana)"
- **registry** — `all_codes()` returns `&'static [&'static str]` instead of allocating `Vec`
- **lib.rs** — Crate documentation updated to reflect five modules (added registry)
- **docs** — Architecture overview rewritten with registry, script/language tables

## [0.2.0] - 2026-03-31

### Added

- **phoneme** — `PhonemeInventoryBuilder` for ergonomic inventory construction with `consonant()`, `vowel()`, `stress()`, `tones()`, and `with_capacity()` methods
- **phoneme** — Sanskrit (Classical) phoneme inventory: 36 consonants + 15 vowels, organized by 5 vargas (consonant groups for Katapayadi encoding) `[S]`
- **phoneme** — Greek (Modern Standard) phoneme inventory: 20 consonants + 5 vowels `[S]`
- **script** — Pre-built metadata for 8 writing systems: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana, Greek `[S]`
- **script** — `by_code()` lookup by ISO 15924 code, `all_codes()` enumeration, `Script::contains_codepoint()` for Unicode range checking
- **registry** — Language registry with ISO 639 lookup: `info()`, `phonemes()`, `primary_script()`, `all_codes()`
- **phoneme** — `LabialVelar` place of articulation variant
- **phoneme** — `Phoneme::consonant()` and `Phoneme::vowel()` constructors for external use
- **phoneme** — `PartialEq`/`Eq` derives on `Phoneme`, `PhonemeKind`, `PhonemeInventory`
- **script** — `PartialEq`/`Eq` derives on `Script`
- **grammar** — `PartialEq`/`Eq` derives on `GrammarProfile`
- **lexicon** — `PartialEq`/`Eq` derives on `LexEntry`, `Lexicon`
- 6 criterion benchmarks (3 inventories, phoneme lookup, registry lookup, script lookup)

### Changed

- **phoneme** — All string fields migrated to `Cow<'static, str>` for zero-alloc static inventories
- **phoneme** — English `/w/` reclassified from `Bilabial` to `LabialVelar` (linguistically accurate)
- **phoneme** — `english()` refactored to use `PhonemeInventoryBuilder`
- **phoneme** — `#[non_exhaustive]` added to `PhonemeKind::Consonant` and `PhonemeKind::Vowel` variants
- **script** — All string fields migrated to `Cow<'static, str>`
- **grammar** — `language_code` migrated to `Cow<'static, str>`
- **lexicon** — All string fields migrated to `Cow<'static, str>`
- Tracing instrumentation added to all public lookup methods

### Performance

- `english_phoneme_inventory`: 146ns (builder with pre-alloc)
- `sanskrit_phoneme_inventory`: 197ns
- `greek_phoneme_inventory`: 90ns
- `phoneme_lookup_ipa`: 14ns
- `registry_phonemes_lookup`: 200ns
- `script_by_code_lookup`: 19ns

## [0.1.0] - 2026-03-30

### Added

- **phoneme** — IPA phoneme inventories with articulatory features (manner, place, voicing, height, backness, rounding), stress patterns, tone systems. English (General American) inventory included
- **script** — Writing system metadata: alphabet, syllabary, logographic, abjad, abugida, mixed. Unicode ranges, directionality (LTR, RTL, TTB, bidi)
- **grammar** — Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order (SVO/SOV/VSO/VOS/OVS/OSV/Free), case systems, gender, dual number, classifiers
- **lexicon** — Lexical entries with IPA transcription, part of speech, frequency ranking, Swadesh list indexing. Lookup, Swadesh extraction, frequency ranking
- **error** — `LipiError` with variants for unknown language/script, missing phonemes, invalid IPA, word-not-found
- **logging** — Optional structured logging via `LIPI_LOG` env var (feature-gated)
- Initial criterion benchmarks for phoneme inventory construction and lookup
