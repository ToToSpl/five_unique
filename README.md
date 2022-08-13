# Five unique

This code is based on [this video](https://www.youtube.com/watch?v=_-AfhLQfb6w). The task is to find five five-letter words which does not share letters (i.e. they use 25 of 26 letters of the alphabet).
I have made this project to learn something about Rust.

## Running 

To run this program you need set of words. You can download **words_alpha.txt** from [here](https://github.com/dwyl/english-words).
You can also process **words_5.ts** file from [wordle repo](https://github.com/MikhaD/wordle/blob/main/src/words_5.ts) to get list of words which makes more sense and are faster to evaluate.

*Important!* remeber to compile the code as the release with:

`cargo run --release`

otherwise it will take much longer to calculate.
