# tbpatch
Token-based patch experiments


This is a small experiment in performing the source code patching at a slightly higher level than just characters:

Namely, both the original text and the "before" and "after" texts of the patch are split into (leading whitespace, token) tuples.

Then the patching is done over these tuples, ignoring the leading whitespace, and finally
the reconstruction of the text is done by emitting the tokens together with their leading whitespaces.

This results in a fairly robust patching:

you should be able to take a diff made against a file formatted using GNU formatting,
and apply it to the same file with Linux formatting.

The usage is similar to "patch" command - either supply the patch filename as the argument,
or feed it via stdin.
