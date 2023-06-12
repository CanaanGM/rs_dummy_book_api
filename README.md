# Dummy API to test rocket

### Exp thus far

- docs are outdated, so looking for a way to customize the port took a while
- unrelated to Rocket, but rust analyzer blocks a resource so u wont be able to build till it finishes.

- you can easily split routes depending on ur needs, same with error catchers
  - error catchers are like a middle ware that handles status code errors, i would think

- Rocket expects u to pass a struct that implements `FromRequest` trait and be passed as a parameter to a route function to be considered as a route guard.

- installing `diesel` on windows is a pain in the rear if u don't have all the deps;



## Deps
1. [diesel_cli](https://crates.io/crates/diesel_cli) 
   - `cargo install diesel_cli`
   - **Create** migration : `diesel migration generate <NAME>` ; u'd need to manually add in the sql tho
   - **List** all migrations for a specific db : `diesel migration list --database-url ./database.sqlite`
   - **Apply**, u need to run ***List*** again to check the proccess :`diesel migration run --database-url ./database.sqlite`
   - **Revert**, u need to run ***List*** again to check the proccess :`diesel migration revert --database-url ./database.sqlite`