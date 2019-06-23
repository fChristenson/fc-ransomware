# fc-ransomware

## What we will cover

* What is ransomware?
* How does it work?

## Notes

Ransomware is when a program encrypts files on a computer and where the hacker demands money for giving
the victim access to their files again.

The usual way to get attacked is that you run a program on your computer and it encrypts all your
personal files, shows some form of message where you need to input a password that will unlock your
files again.

The password will be provided to you after you pay the hacker.

One of the simplest ways to make a ransomware program is to use a XOR bit operation on the victims
files.

A XOR operation simply means that if exactly one of two bits are 1 the result will be 1 otherwise
the result will be 0.

    0101  (5) original number
XOR 0011  (3) secret number
  = 0110  (6) encrypted number

Notice that after encrypting the data we can still reverse the encryption by simply running the
XOR operation on the encrypted number with the same secret number.

This means that as long as we have a secret number we can encrypt and decrypt the data easily.
