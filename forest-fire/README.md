Implement the Drossel and Schwabl definition of the
[forest-fire](https://en.wikipedia.org/wiki/Forest-fire_model) model.

This is a 2d cellular automaton with the following rules:

- A burning cell turns into an empty cell
- A tree will burn if at least one neighbor is burning
- A tree ignites with probability *f* even if no neighbor is burning
- An empty space fills with a tree with probability *p*

The command:

    cargo run ./forest-fire-large.json

Will run simulation of 30 rows by 70 cols, for 1000 steps. Check the
forest-fire-large.json config file for more options.

