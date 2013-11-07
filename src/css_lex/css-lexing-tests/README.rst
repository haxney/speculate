CSS parsing tests
#################

This repository contains implementation-independent test for CSS parsers,
based on the 2013 draft of the `CSS Syntax Level 3`_ specification.

.. _CSS Syntax Level 3: http://dev.w3.org/csswg/css-syntax-3/

The upstream repository for these tests is at
https://github.com/SimonSapin/css-parsing-tests


Importing
=========

The recommended way to use these tests in an implementation
is to import them with git-subtree_.

.. _git-subtree: https://github.com/git/git/tree/master/contrib/subtree

To import the first time to a ``./css-parsing-tests`` sub-directory,
run this from the top-level of a git repository::

    git subtree add -P css-parsing-tests https://github.com/SimonSapin/css-parsing-tests.git master

Later, to merge changes made in the upstream repository, run::

    git subtree pull -P css-parsing-tests https://github.com/SimonSapin/css-parsing-tests.git master


Test files
==========

CSS Syntax specification describes a number of "functions".
Each ``.json`` file in this repository corresponds to such a function.
The files are encoded as UTF-8
and each contain a JSON array with an even number of items,
where each pair of items is one function input
associated with the expected result.

``tokens.json``
    Tests `Tokenizer Algorithms
    <http://dev.w3.org/csswg/css-syntax/#tokenizer-algorithms>`_.
    The Unicode input is represented by a JSON string,
    the output as an array of `token values`_ as described below.


Result representation
=====================

Tokenizer nodes (the results of lexing) are represented in JSON as follow.
This representation was chosen to be compact
(and thus less annoying to write by hand)
while staying unambiguous.
For example, the difference between ``@import`` and ``\@import`` is not lost:
they are represented as ``["at-keyword", "import"]`` and ``["ident", "@import"]``,
respectively.

.. _token value:
.. _token values:

Token values
----------------

<ident>
    Array of length 2: the string ``"ident"``, and the value as a
    string.

<function>
    Array of length 2: the string ``"function"`` and the name of the
    function as a string.

<at-keyword>
    Array of length 2: the string ``"at-keyword"``, and the value as a
    string.

<hash>
    Array of length 3: the string ``"hash"``, the value as a string, and
    the type as the string ``"id"`` or ``"unrestricted"``.

<string>
    Array of length 2: the string ``"string"``, and the value as a
    string.

<bad-string>
    The array of two strings ``["error", "bad-string"]``.

<url>
    Array of length 2: the string ``"url"``, and the value as a string.

<bad-url>
    The array of two strings ``["error", "bad-url"]``.

<delim>
    The value as a one-character string.

<number>
    Array of length 4: the string ``"number"``, the representation as a
    string, the value as a number, and the type as the string
    ``"integer"`` or ``"number"``.

<percentage>
    Array of length 4: the string ``"percentage"``, the representation
    as a string, the value as a number, and the type as the string
    ``"integer"`` or ``"number"``.

<dimension>
    Array of length 4: the string ``"dimension"``, the representation as
    a string, the value as a number, the type as the string
    ``"integer"`` or ``"number"``, and the unit as a string.

<unicode-range>
    Array of length 3: the string ``"unicode-range"``, followed by the
    *start* and *end* integers as two numbers.

<include-match>
    The string ``"~="``.

<dash-match>
    The string ``"|="``.

<prefix-match>
    The string ``"^="``.

<suffix-match>
    The string ``"$="``.

<substring-match>
    The string ``"*="``.

<column>
    The string ``"||"``.

<whitespace>
    The string ``" "`` (a single space.)

<CDO>
    The string ``"<!--"``.

<CDC>
    The string ``"-->"``.

<colon>
    The string ``":"``.

<semicolon>
    The string ``";"``.

<comma>
    The string ``","``.

<{>
    The string ``"{"``.

<}>
    The string ``"}"``.

<(>
    The string ``"("``.

<)>
    The string ``")"``.

<[>
    The string ``"["``.

<]>
    The string ``"]"``.
