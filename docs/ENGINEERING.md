# Engineering rules

1. No comments in code files. No emojis in code or web UI. Names are self-documenting.
2. Functions stay roughly under 40 lines. Split larger ones into reusable, program-agnostic pieces with clear inputs and outputs.
3. Files stay roughly under 200 lines. Split any that reach about 400 into logically separate files.
4. All values are externalized. SQL in `.sql`, prompts in templates, mappings in `.json`, settings in config files. No long inline strings, no large mapping tables in code.
5. No repeated strings. Use constants for status values and the like.
6. No repeated code. Shared logic goes in a utils module. Write a pattern twice, extract it.
7. No magic numbers or fixed values. Expose them as function arguments or config.
8. No raw dictionaries passed between functions. Use structs or data classes unless it adds noise.
9. Keep nesting to two or three levels. Use early returns. Prefer a different mechanism (tables, polymorphism, pipelines) over nested branches.
10. Clean up temporary files. Scripts live in `scripts/`.
