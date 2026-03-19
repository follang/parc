# Location Tracking

The `loc` module handles mapping byte offsets in preprocessed source back to original file locations.

## How it works

The C preprocessor emits line markers of the form:

```
# linenum "filename" flags
```

PAC parses these markers to build a location map. Given a byte offset in the preprocessed source, `get_location_for_offset` returns the original file and line number.

## Span

Every AST node is wrapped in a `Node<T>` that carries a `Span` — a byte range in the preprocessed source. This allows tools (like BIC) to trace declarations back to their origin files.
