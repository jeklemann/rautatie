# Rautatie

This program conjugates verbs in Finnish, as well as forming the base forms of participles.

## Structure
- verb: Contains the verb structure itself, and the logging feature.
- wiktionary: Contains code to fetch words and parsing their fi-conj macros.
- menu: All CLI interaction is based here.
- grammar: This area contains all of the grammatical rules.
    - grammar/transforms: This has general transformations or ones that are a little large. This file may be slightly messy.
    - grammar/moods: This contains all of the implemented moods. Each mood has tense modules, which have all the functions to form verbs.
    - grammar/participles: This contains all the participle functions.
    - grammar/dispatch: This contains code for dispatching verb functions from given choices.

## Features
- Guessing game to guess the form of a verb
- Step by step process to create verb forms

## Currently unimplemented
- Non-lemma infinitives
- Potential

## Improvements
- A lot of the perfect code is just duplicated code
- The tenses could maybe be moved into a trait, which may help with facilitating the above

## Bugs
- Yes
