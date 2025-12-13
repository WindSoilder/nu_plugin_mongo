# nu_plugin_mongo

A nushell plugin to interactive with [mongodb](https://www.mongodb.com).

Sometimes I want to explore mongodb data, but I don't want to open `mongo` shell or `python` to query mongodb data.

I want to have more lite weight tool to explore data.  So here is the plugin.

Starts from 0.1.9, it support auto-completions for collection names, and it requires nushell version to be 0.109.

## Demo

<p align="center">
  <img width="600" src="demo.svg">
</p>

## Note

- All I want to do is query and explore data, so the plugin includes query commands only.
- You might note that the command prefix is `mongoc`, not `mongo`.  Because I don't want to make the name conflict with official lagacy [mongo](https://www.mongodb.com/docs/manual/reference/mongo/) cli.

## Usage

1. First, you need to have latest nushell installed(Of cause it's a nushell plugin).
2. Then you can install from `cargo`
```
cargo install nu_plugin_mongo
```
3. Add and use the plugin: `plugin add <nu_plugin_mongo_path>; plugin use mongo`
4. Then you will gain some `mongoc` commands to play with.


## Full help

```nushell
Operate with mongodb

You must use one of the following subcommands. Using this command as-is will only produce this help message.

Usage:
  > mongoc

Subcommands:
  mongoc count (plugin) - count mongodb documents
  mongoc delete-many (plugin) - delete many mongodb documents
  mongoc delete-one (plugin) - delete one mongodb document
  mongoc drop (plugin) - drop a mongodb collection
  mongoc estimated (plugin) - estimated mongodb documents count
  mongoc find (plugin) - find mongodb documents
  mongoc find-one (plugin) - find mongodb documents
  mongoc list (plugin) - list mongodb connections
  mongoc list-colls (plugin) - list all available collection names
  mongoc list-indexes (plugin) - find mongodb documents
  mongoc open (plugin) - open mongodb connection, the url must contains default databse
  mongoc remove (plugin) - remove mongodb handles
  mongoc select (plugin) - select current mongodb handle

Flags:
  -h, --help: Display the help message for this command

Input/output types:
  ╭───┬─────────┬────────╮
  │ # │  input  │ output │
  ├───┼─────────┼────────┤
  │ 0 │ nothing │ string │
  ╰───┴─────────┴────────╯
```
