# Dummy API to test rocket

This is a basic app for practice and learning,  
### Exp thus far

- docs are outdated, so looking for a way to customize the port took a while
- unrelated to Rocket, but rust analyzer blocks a resource so u wont be able to build till it finishes.

- you can easily split routes depending on ur needs, same with error catchers
  - error catchers are like a middle ware that handles status code errors, i would think

- Rocket expects u to pass a struct that implements `FromRequest` trait and be passed as a parameter to a route function to be considered as a route guard.

- installing `diesel` on windows is a pain in the rear if u don't have all the deps;

- configuring the PORT in Rocket.toml didn't work yesterday, so i had to use the custom config programatically which when i added a DB screwed things over and wasted what remained of my sanity debugging it, today i just added the config and it worked . . . js all over again xD
  - unclear docs are unclear tho

## Deps
1. [diesel_cli](https://crates.io/crates/diesel_cli) 
   - `cargo install diesel_cli`
   - **diesel setup** : to init the schema of ur chosen db, ***READ*** what is generated 
   - **Create** migration : `diesel migration generate <NAME>` ; u'd need to manually add in the sql tho
   - **List** all migrations for a specific db : `diesel migration list --database-url ./database.sqlite`
   - **Apply**, u need to run ***List*** again to check the proccess :`diesel migration run --database-url ./database.sqlite`
   - **Revert**, u need to run ***List*** again to check the proccess :`diesel migration revert --database-url ./database.sqlite`


## docs used:
- https://rocket.rs/v0.5-rc/
- https://rocket.rs/v0.5-rc/guide/state/
- https://diesel.rs/guides/getting-started
- https://api.rocket.rs/master/rocket_db_pools/
- [setup ](https://phoenixnap.com/kb/postgres-create-user) postgres user and tables
- reddit is protesting so it's useless
- [vault warden](https://github.com/dani-garcia/vaultwarden/blob/main/Cargo.toml) for reference
- [stackoverflow](https://stackoverflow.com/questions/49385326/how-do-i-fix-mismatching-dependencies-in-my-cargo-file-to-work-around-native-lib), not as much as i thought i would 