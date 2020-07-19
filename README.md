# wikipedia_roots

What this **Sunday program** does is to follow through the first link of a Wikipedia article, starting from a random one, up to a target article, a max steps, or a dead end.

It is inspired by this [Getting to Philosophy](https://en.wikipedia.org/wiki/Wikipedia:Getting_to_Philosophy) article.

Let's follow the random to the roots or whatever!

## How to use it

First thing first, you may want to build it:

```
$ cargo build
```

Then you can run it with no parameters and it assumes that you want to get to Philosophy:

```
$ target/debug/wikipedia_roots
Starting at: https://en.wikipedia.org/wiki/Special:Random
Target is: https://en.wikipedia.org/wiki/Philosophy
Starting...
https://en.wikipedia.org/wiki/Special:Random
https://en.wikipedia.org/wiki/Kerala
https://en.wikipedia.org/wiki/Help:IPA/English
https://en.wikipedia.org/wiki/International_Phonetic_Alphabet
https://en.wikipedia.org/wiki/Alphabet
https://en.wikipedia.org/wiki/Symbols
https://en.wikipedia.org/wiki/Word
https://en.wikipedia.org/wiki/Linguistics
https://en.wikipedia.org/wiki/Science
https://en.wikipedia.org/wiki/Latin
https://en.wikipedia.org/wiki/Help:IPA/Latin
Got to an article visited before. (abort)
> https://en.wikipedia.org/wiki/International_Phonetic_Alphabet
```

Or you can give it a target article to try and reach for:

```
$ target/debug/wikipedia_roots https://en.wikipedia.org/wiki/Science
Starting at: https://en.wikipedia.org/wiki/Special:Random
Target is: https://en.wikipedia.org/wiki/Science
Starting...
https://en.wikipedia.org/wiki/Special:Random
https://en.wikipedia.org/wiki/Natural_number
https://en.wikipedia.org/wiki/Mathematics
https://en.wikipedia.org/wiki/Ancient_Greek
https://en.wikipedia.org/wiki/Greek_language
https://en.wikipedia.org/wiki/Western_world
https://en.wikipedia.org/wiki/Region
https://en.wikipedia.org/wiki/Geography
Got to an article visited before. (abort)
> https://en.wikipedia.org/wiki/Ancient_Greek
```

## Why?

Well, who doesn't like to put on some Rust code in a beautiful quarantine's Sunday?
