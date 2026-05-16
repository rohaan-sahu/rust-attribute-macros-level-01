### This 'SKILL' was provided to claude chat and this project was built with help from the LLM chat bot.
The goal was to gain a conceptual understanding about the workings of macros in Rust.
It was a great tutoring tool as it could answer specific questions that I had.
This was done on a free account, so I did run out of the daily limit once.

---

**Attribute Macro Learning Ruleset**

```
Topic: Rust attribute macros using syn + quote
Rules:
- Start with simple function transformation
- Each example builds ONE new concept
- Progression order:
  1. Attribute: wrap function with println before/after
  2. Attribute: read function name, log it
  3. Attribute: modify function return value
  4. Attribute: accept single parameter #[my_macro(timeout=5)]
  5. Attribute: accept multiple parameters
  6. Attribute: apply to structs, not functions
  7. Attribute: combine struct + function transformations
- Every example must:
  - Show lib.rs (macro definition)
  - Show main.rs (usage)
  - Explain ONE new concept only
  - Be under 35 lines total
  - Show generated code in comments
- No skipping steps
- State explicitly what's new vs previous
```
