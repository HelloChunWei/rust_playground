# coding challenge 01:

reference: https://codingchallenges.fyi/challenges/challenge-wc/

## step one:
In this step your goal is to write a simple version of wc, let’s call it ccwc (cc for Coding Challenges) that takes the command line option -c and outputs the number of *bytes* in a file.
```
cargo run -c test.txt
```

answer: `335039 test.txt`

## step two:
In this step your goal is to support the command line option -l that outputs the number of *lines* in a file.
```
cargo run -l test.txt
```
answer: `7142 test.txt`


## step three:
In this step your goal is to support the command line option -w that outputs the number of *words* in a file. If you’ve done it right your output should match this:
```
cargo run -w test.txt
```
answer: `58164 test.txt`


## Step Four:
In this step your goal is to support the command line option -m that outputs the number of *characters* in a file. If the current locale does not support multibyte characters this will match the -c option.
```
cargo run -m test.txt
```
answer: `332143 test.txt`

## Step Five:
In this step your goal is to support the default option - i.e. no options are provided, which is the equivalent to the -c, -l and -w options. If you’ve done it right your output should match this:

```
cargo run test.txt
```
answer: `7142  58164  335039 test.txt`


## Final: (not sure how to implement?)
In this step your goal is to support being able to read from standard input if no filename is specified. If you’ve done it right your output should match this:

```
cat test.txt | ccwc -l
```

answer: `7142`