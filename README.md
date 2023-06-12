# Dummy API to test rocket

### Exp thus far

- docs are outdated, so looking for a way to customize the port took a while
- unrelated to Rocket, but rust analyzer blocks a resource so u wont be able to build till it finishes.

- you can easily split routes depending on ur needs, same with error catchers
  - error catchers are like a middle ware that handles status code errors, i would think

- Rocket expects u to pass a struct that implements `FromRequest` trait and be passed as a parameter to a route function to be considered as a route guard.





## Deps
1. [diesel_cli](https://crates.io/crates/diesel_cli) 
   - `cargo install diesel_cli`