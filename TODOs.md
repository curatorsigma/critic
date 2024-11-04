# Renaming
- rename all types to fit the module they are currently in
    - e.g. flattened..., tokenized... etc.

# Words split over multiple folios
- there could be words split over two folios
- if this happens: mark the part of the word on the first folio with the lex data for the
  entire word and mark the part of the word on the second folio with
  "second_half_of_cross_folio_break = true"
     - words with this marker that appear as the first word of a folio will be ignored
       completely when reading in lex
     - words with this marker anywhere else will raise an error
- rationale: We want to lex based on folio, not on witness, to keep file sizes small. Since
  we have to faithfully represent each folio that means we will have partial words on some
  occasions. We can ignore them while reading the filled lex file back


# propose lex and morph data
- Add expandability to have the lex output make proposals for lex and morph data
- automatically suggest the lex and morph for punctuation

# reading lex files from disk into a LexedFolioTranscript

